use tailwag::macros::derive_magic;

derive_magic! {
    pub struct Todo {
        id: uuid::Uuid, // Note: Currently all data types MUST have an `id` of type `uuid::Uuid`. A future version will remove this limitation.
        title: String,
        description: String,
        due_date: chrono::NaiveDateTime,
    }
}
