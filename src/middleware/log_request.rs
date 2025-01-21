use std::{future::Future, pin::Pin, sync::Arc};

use log;
use tailwag::{prelude::*, web::application::NextFn};
pub(crate) fn log_request_middleware(
    req: Request,
    ctx: RequestContext,
    next: Arc<NextFn>,
) -> Pin<Box<dyn Send + Future<Output = Response>>> {
    Box::pin(async move {
        log::info!("Received a request: {:?}", &req);
        let res = next(req, ctx).await;
        println!("Finished request");
        res
    })
}
