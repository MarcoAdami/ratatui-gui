use omc_galaxy::{ExplorerStatusNotLock, Orchestrator, PlanetInfoMap};
use std::time::{Duration, Instant};

use crate::game_state::GameState;
use omc_galaxy::settings;

pub struct App {
    pub(crate) gamestate: GameState,
    pub(crate) orchestrator: Orchestrator,
    pub(crate) planets_info: PlanetInfoMap, //Planet Info
    pub(crate) explorers: ExplorerStatusNotLock,
    pub(crate) probability_sunray: u32,

    pub(crate) exit: bool,
    pub(crate) tick_rate: Duration,
    pub(crate) last_tick: Instant,
    pub(crate) frame_rate: Duration, // Useful not to overload the CPU
}

impl App {
    pub fn new(orchestrator: Orchestrator) -> Self {
        Self {
            gamestate: GameState::WaitingStart,
            planets_info: orchestrator.get_planets_info(),
            explorers: orchestrator.get_explorer_states(),
            orchestrator,
            probability_sunray: settings::get_sunray_probability(),

            exit: false,
            last_tick: Instant::now(),
            tick_rate: Duration::from_millis(200),
            frame_rate: Duration::from_millis(33), // UI fluida a 30 FPS
        }
    }

    pub fn get_game_state(&self) -> GameState {
        self.gamestate.clone()
    }

    pub fn set_game_state(&mut self, state: GameState) {
        self.gamestate = state;
    }

    pub fn initialize_by_file(&mut self) -> Result<(), String> {
        // Load env
        dotenv::dotenv().ok();

        // Give the absolute path for the init file
        let file_path =
            std::env::var("INPUT_FILE").map_err(|_| "Set INPUT_FILE in .env or env vars")?;

        self.orchestrator
            .initialize_galaxy_by_file(file_path.as_str().trim())
            .map_err(|_| "Failed to initialize galaxy")?;
        Ok(())
    }

    pub(crate)fn set_sunray_increment(&mut self){
        settings::set_sunray_probability(self.probability_sunray+1);
    }
    pub(crate)fn set_sunray_decrement(&mut self){
        settings::set_sunray_probability(self.probability_sunray-1);
    }
}
