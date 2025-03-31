mod app;
mod events;
mod pokemon;
use app::App;
use color_eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // console_subscriber::init();
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::default().run(terminal).await;
    ratatui::restore();
    app_result
}