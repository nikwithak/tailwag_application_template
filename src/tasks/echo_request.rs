use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct EchoRequestEvent {
    pub data: String,
}

pub async fn echo_request_task(event: EchoRequestEvent) {
    let EchoRequestEvent {
        data,
    } = event;
    log::info!("Processed EchoRequestEvent: {:?}", data);
}
