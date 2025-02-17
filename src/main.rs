use app::App;

mod app;
mod error;
mod logging;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = App::new();
    app.run();

    Ok(())
}
