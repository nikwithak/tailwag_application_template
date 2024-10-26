use tailwag_application_template::MyTailwagApplication;
#[tokio::main]
async fn main() {
    MyTailwagApplication::build_new()
        .run()
        .await
        .expect("Service did not exit cleanly.");
}
