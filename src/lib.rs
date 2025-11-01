mod middleware;
mod rest_endpoints;
mod tasks;

use middleware::log_request::log_request_middleware;
use rest_endpoints::todo::Todo;
use tailwag::web::{
    application::{WebService, WebServiceBuildResponse},
    extras::comment::Comment,
};

use crate::rest_endpoints::todo::TodoFile;

pub struct MyTailwagApplication;
impl MyTailwagApplication {
    pub fn build_new() -> WebServiceBuildResponse {
        WebService::builder("MyTailwagApplication")
            .with_resource::<TodoFile>() // Even though TodoFile is a child of Todo, it must be registered with the service to make sure tables are created. Current limitation - be sure to mark the table with `#[no_default_routes]`!
            .with_resource::<Todo>() // `with_resource::<T>()` adds the type T to Tailwag's registry. Its database table and associated routes will be added automatically to the root.
            .with_resource::<Comment>()
            .with_authentication() // Adds the default authentication module, including middleware, data types, and authentication routes.
            .with_middleware(log_request_middleware) // Middleware are executed top down - each one wraps the next. Use middleware to handle Request pre-processing, and Response post-processing.
            .with_task(tasks::echo_request::echo_request_task) // Tasks can receive messages to queue for processing. Experimental: Use with caution
            .with_static_files() // A built-in extra, for serving files from the `./static` directory. Markdown is rendered to HTML.
            .build_service()
    }
}
