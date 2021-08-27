use anyhow::Result;
use tv::Application;

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = Application::new();
    app.run().await
}
