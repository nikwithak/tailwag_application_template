use chrono::Utc;
use serde::{Deserialize, Serialize};
use tailwag::macros::{derive_magic, Display};
use tailwag::orm::data_manager::local_storage_provider::LocalStorageFileProvider;
use tailwag::orm::data_manager::traits::DataProvider;
use tailwag::orm::queries::Insertable;
use tailwag::prelude::*;
use tailwag::web::application::http::multipart::FromMultipartRequest;
use tailwag::web::auth::gateway::{AppUser, Session};
#[allow(deprecated)] // post_comment: Verified working in this instance.
use tailwag::web::extras::comment::{post_comment, Comment, Commentable};
use tailwag::web::extras::file_upload::File;
use tailwag::web::extras::mime_type::MimeType;
use tailwag::web::option_utils::OrError;
use uuid::Uuid;

// Tailwag Tutorial: A basic item with file attachments, user-comments, and basic user tracking.

// The [derive_magic!] macro gives us all the database and API logic.
derive_magic! {
    // We use #[post()] to override the default POST operation. In this case, needed to handle the user_created logic (see Todo::create)
    #[post(Todo::create)]
    //  #[actions()] defines additional POST routes. `"/{id}/"` will match any string and provide it as a path_param in the [Request].`
    #[actions(
        //  The macro accepts a set of tuples as parameters - each tuple pairs a path to a function.
        ("/{id}/comment", post_todo_comment),
        ("/{id}/file", upload_file),
        ("/{id}/file/{id}", get_file),
    )]
    pub struct Todo {
        id: uuid::Uuid, // All Tailwag-compatible data types must have an `id` of type `uuid::Uuid`.

        title: String, // Basic data types will be automatically added to the database and API request types.
        description: Option<String>,
        created_date: chrono::NaiveDateTime,
        #[ref_only] // #[ref_only] is used to establish a non-ownership relationship in the database. i.e. [AppUser] is not owned by [Todo].
        created_by: AppUser,
        #[string] // #[string] tells tailwag to treat this data type as a string.
        #[no_filter] // #[no_filter] tells tailwag to disable filtering on this field - we cannot filter on "Status", due to current limitations with tailwag enums.
        status: Status,
        #[create_ignore] // #[create_ignore] - leaves this field out of the create request types.
        #[no_filter]
        files: Vec<TodoFile>,
        due_date: Option<chrono::NaiveDateTime>,
        #[no_filter]
        #[serde(default)]
        comment: Vec<Comment>, // Comment is a module from `tailwag::web::extras`
    }
}

#[derive(Clone, Debug, Default, Display, Serialize, Deserialize)]
pub enum Status {
    #[default]
    NotStarted,
    InProgress,
    Paused,
    Review,
    Done,
}

#[derive(Serialize, Deserialize)]
pub struct CreateTodoRequest {
    title: String,
    description: Option<String>,
    due_date: Option<chrono::NaiveDateTime>,
}

pub async fn post_todo_comment(
    req: Request,
    ctx: RequestContext,
) -> HttpResult<Comment> {
    #[allow(deprecated)]
    post_comment::<Todo>(req, ctx).await
}
impl Commentable for Todo {
    fn add_comment(
        &mut self,
        comment: tailwag::web::extras::comment::Comment,
    ) {
        self.comment.push(comment);
    }
}

impl Todo {
    /// POST operation to set created_date / created_by details.
    /// TODO: Eventually this will be moved to the BuildRoutes macros
    pub async fn create(
        req: CreateTodoRequest,
        todos: PostgresDataProvider<Todo>,
        users: PostgresDataProvider<AppUser>,
        session: Option<Session>,
    ) -> HttpResult<Self> {
        type CreateRequest = <Todo as Insertable>::CreateRequest;
        let user = session.or_404()?.get_current_user(users).await?.or_404()?;

        Ok(todos
            .create(CreateRequest {
                title: req.title,
                description: req.description,
                created_by: user,
                created_date: Utc::now().naive_utc(),
                due_date: req.due_date,
                status: Default::default(),
                comment: Default::default(),
            })
            .await?)
    }
}

derive_magic! {
    #[no_default_routes]
    pub struct TodoFile {
        id: Uuid,
        file_key: String,
    }
}

impl FromMultipartRequest for <TodoFile as Insertable>::CreateRequest {
    fn from_multipart_request(
        _: &tailwag::web::application::http::multipart::MultipartParts
    ) -> Result<Self, HttpError> {
        Ok(Self {
            ..Default::default()
        })
    }
}

/// Uploads a file to the specified Todo item.
pub async fn upload_file(
    req: Request,
    ctx: RequestContext,
) -> HttpResult<TodoFile> {
    let id = req.path_params.first().or_404()?.clone();
    let todos = ctx.get::<Todo>().or_404()?;
    let file: File<<TodoFile as Insertable>::CreateRequest> =
        <_ as tailwag::prelude::FromRequest>::from(req)?;
    let ServerData(files): ServerData<LocalStorageFileProvider> =
        ServerData::<LocalStorageFileProvider>::from(&ctx);

    let inferred_mime_type =
        MimeType::try_from_filename(file.filename.as_str()).unwrap_or_default();
    if file.mime_type != inferred_mime_type {
        HttpError::bad_request("File type doesn't match provided mime_type")?;
    }

    let id: Uuid = Uuid::parse_str(&*id).ok().or_404()?;
    let mut todo_item = todos.get(|item| item.id.eq(id)).await?.or_404()?;

    files.save_file(&file.filename, file.bytes)?;

    let file_item = TodoFile {
        id: Uuid::new_v4(),
        file_key: file.filename.clone(),
    };
    todo_item.files.push(file_item.clone());

    todos.update(&todo_item).await?;

    Ok(file_item)
}

pub async fn get_file(
    Request {
        mut path_params,
        ..
    }: Request,
    todos: PostgresDataProvider<Todo>,
    ServerData(files): ServerData<LocalStorageFileProvider>,
) -> HttpResult<Response> {
    // Tailwag Tutorial: Because we are extracting multiple PathParams, we have to manually fetch them from the request.
    // This is due to a limitation in tailwag's `IntoRouteHandler`.
    let item_id = path_params.pop().and_then(|id| Uuid::parse_str(&id).ok()).or_404()?;
    let file_id = path_params.pop().and_then(|id| Uuid::parse_str(&id).ok()).or_404()?;

    let todo = todos.get(|i| i.id.eq(item_id)).await?.or_404()?;
    let file_data = todo.files.iter().find(|f| f.id.eq(&file_id)).or_404()?;

    let bytes = files.read_file(&file_data.file_key)?;

    Ok(Response::ok().with_body(bytes).with_header(
        "content-type",
        MimeType::try_from_filename(&file_data.file_key)
            .map(|mt| mt.to_string())
            .unwrap_or("application/octet-stream".to_string()),
    ))
}
