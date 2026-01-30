mod app;
mod game_state;
mod loops;
mod ui;

use app::App;
use omc_galaxy::Orchestrator;

fn main() -> Result<(), String> {
    // Init terminal
    let mut terminal = ratatui::init();

    // Init orchestrator
    let orchestrator = Orchestrator::new()?;

    // Create and run game loop
    let mut app = App::new(orchestrator);

    // Initialize by file
    app.initialize_by_file()?;

    // Start the app
    let result = app.run(&mut terminal);

    // Restore terminal
    ratatui::restore();

    // Return possible error from the app run
    result
}
