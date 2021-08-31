use crate::application::Application;
use anyhow::Result;

mod application;
mod input;

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = Application::new();
    app.run().await
}
