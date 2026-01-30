mod main_screen;
mod screens;

use ratatui::Frame;

use crate::app::App;
use crate::game_state::GameState;

pub fn render_ui(app: &App, frame: &mut Frame) {
    match app.get_game_state() {
        GameState::WaitingStart => {
            // Show start screen
            screens::render_start_screen(app, frame);
        }
        GameState::Running => {
            // Show normal game UI
            main_screen::render_game_ui(app, frame);
        }
        GameState::Paused => {
            // Show game UI with pause overlay
            main_screen::render_game_ui(app, frame);
            screens::render_pause_overlay(app, frame);
        }
    }
}
