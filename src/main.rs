mod app;
mod config;
mod game;
mod hack;
mod rain;
mod score;
mod speedrunner;
mod ui;

use color_eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = app::App::new().run(terminal);
    ratatui::restore();
    result
}
