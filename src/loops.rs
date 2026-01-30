use omc_galaxy::settings::get_sunray_probability;
use ratatui::DefaultTerminal;
use std::time::{Duration, Instant};

use crate::app::App;
use crate::game_state::{GameState, handle_game_state};
use crate::ui::render_ui;

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<(), String> {
        while !self.exit {
            match self.get_game_state() {
                GameState::WaitingStart => self.waiting_loop(terminal)?,
                GameState::Running => self.active_loop(terminal)?,
                GameState::Paused => self.paused_loop(terminal)?,
            }
        }
        Ok(())
    }

    /// Loop dedicated exclusively to the initial waiting phase
    fn waiting_loop(&mut self, terminal: &mut DefaultTerminal) -> Result<(), String> {
        // Draw the start screen
        terminal
            .draw(|frame| render_ui(self, frame))
            .map_err(|_| "Error while drawing start screen")?;

        // Wait for user input (Start or Quit)
        handle_game_state(self)?;
        Ok(())
    }

    /// Loop: tick management and orchestrator
    fn active_loop(&mut self, terminal: &mut DefaultTerminal) -> Result<(), String> {
        let mut last_frame = Instant::now();

        while !self.exit && self.gamestate == GameState::Running {
            // --- 1. DISEGNO (Solo se è passato il tempo del frame_rate) ---
            if last_frame.elapsed() >= self.frame_rate {
                terminal
                    .draw(|frame| render_ui(self, frame))
                    .map_err(|_| "Error drawing UI")?;
                last_frame = Instant::now();
            }

            // --- 2. INPUT UTENTE (Non bloccante, timeout brevissimo) ---
            // Usiamo un timeout minuscolo per non bloccare il resto della logica
            // Attualmente non sembra necessario un timeout così breve, ma è una buona pratica
            handle_game_state(self)?;

            // --- 3. GESTIONE MESSAGGI (Continua) ---
            // Processiamo piccoli batch ad ogni iterazione del loop
            self.orchestrator.handle_game_messages()?;

            // --- 4. TICK LOGICA (Eventi Spaziali) ---
            if self.last_tick.elapsed() >= self.tick_rate {
                self.get_game_info();
                self.orchestrator.send_sunray_or_asteroid()?;
                self.last_tick = Instant::now();
            }

            // --- 5. RIPOSO (Opzionale ma consigliato) ---
            // Un piccolo sleep per non bruciare la CPU se il loop è troppo veloce
            std::thread::sleep(Duration::from_millis(1));
        }
        Ok(())
    }

    fn get_game_info(&mut self){
        self.planets_info = self.orchestrator.get_planets_info();
        self.probability_sunray = get_sunray_probability();
    }

    /// Pause loop: only consume UI messages, time frozen
    fn paused_loop(&mut self, terminal: &mut DefaultTerminal) -> Result<(), String> {
        // Draw the pause overlay
        terminal
            .draw(|frame| render_ui(self, frame))
            .map_err(|_| "Error while drawing pause screen")?;

        // Wait for user input
        handle_game_state(self)?;
        Ok(())
    }
}
