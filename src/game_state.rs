use crate::app::App;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use omc_galaxy::{Game, Orchestrator};
use std::time::Duration;

#[derive(Clone, PartialEq, Debug)]
pub enum GameState {
    WaitingStart,
    Running,
    Paused,
    Ended
}

impl GameState{
    pub(crate) fn is_running(&self)->bool{
        *self == GameState::Running
    }
    pub(crate) fn is_paused(&self)->bool{
        *self == GameState::Paused
    }
    
}

pub fn handle_game_state(app: &mut App) -> Result<(), String> {
    // Timeout molto breve per input reattivo
    if event::poll(Duration::from_millis(10)).map_err(|_| "Polling error")? {
        if let Event::Key(key) = event::read().map_err(|_| "Reading events error")? {
            match (key.code, app.get_game_state()) {
                // Eventi che rispondono alla pressione (immediati)
                (KeyCode::Char('q'), _) => {
                    app.exit = true;
                }
                (KeyCode::Enter, GameState::WaitingStart) => {
                    app.set_game_state(GameState::Running);
                    app.orchestrator.start_all()?;
                }
                (KeyCode::Char('p'), GameState::Running) => {
                    app.set_game_state(GameState::Paused);
                }
                (KeyCode::Char('p'), GameState::Paused) => {
                    app.set_game_state(GameState::Running);
                }
                (KeyCode::Up, _)=>{
                    app.set_sunray_increment()
                }
                (KeyCode::Down, _)=>{
                    app.set_sunray_decrement()
                }
                
                // Eventi di navigazione: rispondono al RILASCIO per evitare ripetizioni
                (KeyCode::Char('w'), _) => {
                    app.decrement_id_selector();
                }
                (KeyCode::Char('s'), _) => {
                    app.increment_id_selector();
                }
                
                // Toggle log overlay con 'L'
                (KeyCode::Char('l'), _) => {
                    app.show_log_overlay = !app.show_log_overlay;
                }
                // (KeyCode::Char('r'), GameState::Ended)=>{
                //     app.set_game_state(GameState::WaitingStart);
                //     app.orchestrator = Orchestrator::new()?;
                // }
                _ => {}
            }
        }
    }
    Ok(())
}