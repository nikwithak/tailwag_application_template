mod middleware;
mod rest_endpoints;
mod tasks;

use middleware::log_request::log_request_middleware;
use rest_endpoints::todo::Todo;
use tailwag::web::application::{WebService, WebServiceBuildResponse};

pub struct MyTailwagApplication;
impl MyTailwagApplication {
    pub fn build_new() -> WebServiceBuildResponse {
        WebService::builder("MyTaiilwagApplication")
            .with_resource::<Todo>()
            .with_middleware(log_request_middleware)
            .with_task(tasks::echo_request::echo_request_task)
            .with_static_files()
            .build_service()
    }
}
