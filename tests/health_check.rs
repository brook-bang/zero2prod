use std::io::Result;

#[tokio::test]
async fn health_check_works() {
    spawn_app().await.expect("Failed to spawn our app.");
}

async fn spawn_app() -> Result<()> {
    todo!()
}
