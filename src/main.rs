
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Cell, Paragraph, Row, Table},
};
use std::{ io, time::{Duration, Instant}
};

use omc_galaxy::{ExplorerStatusNotLock, Orchestrator, PlanetStatusNotLock, Status};

mod states;
use states::GameState;



pub struct App {
    gamestate: GameState,
    orchestrator: Orchestrator,
    planets: PlanetStatusNotLock,
    explorers: ExplorerStatusNotLock,
    exit: bool,
    tick_rate: Duration,
    last_tick: Instant,
}
impl App{
    fn get_game_state(&self) -> GameState {
        self.gamestate.clone()
    }
    fn set_game_state(&mut self, state: GameState) {
        self.gamestate = state;
    }
}
impl App {
    fn new(orchestrator: Orchestrator) -> Self {
        Self {
            gamestate: GameState::WaitingStart,
            planets: orchestrator.get_planet_states(),
            explorers: orchestrator.get_explorer_states(),
            orchestrator: orchestrator,
            exit: false,
            tick_rate: Duration::from_millis(100),
            last_tick: Instant::now(),
        }
    }
}
// Main Application Loop
impl App {
    pub fn inititialize_by_file(&mut self) -> Result<(), String> {
        // Load env
        dotenv::dotenv().ok();
        //Give the absolute path for the init file
        let file_path =
            std::env::var("INPUT_FILE").map_err(|_| "Set INPUT_FILE in .env or env vars")?;

        self.orchestrator
            .initialize_galaxy_by_file(file_path.as_str().trim())
            .map_err(|_| "Failed to initialize galaxy")?;
        Ok(())
    }

}

// Loop Functionality
impl App{

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<(),String> {
        while !self.exit {
            match self.get_game_state() {
                GameState::WaitingStart => self.waiting_loop()?,
                GameState::Running => self.active_loop(terminal)?,
                GameState::Paused => self.paused_loop()?,
            }
        }
        Ok(())
    }


    /// Loop dedicato esclusivamente alla fase di attesa iniziale
    fn waiting_loop(&mut self) -> Result<(), String> {
        // Qui non facciamo calcoli di tempo, aspettiamo solo lo Start
        self.handle_events().map_err(|_|"Handle events error")?;
        
        Ok(())
    }

    /// Loop ad alte prestazioni: gestione tick e orchestrator
    fn active_loop(&mut self, terminal: &mut DefaultTerminal) -> Result<(), String> {
        while !self.exit {
            // 1. Disegna sempre lo stato attuale (a 60+ FPS o quanto permesso dal terminale)
            terminal.draw(|frame| self.render_ui(frame)).map_err(|_|"Error meanwhile drawing UI")?;

            // 2. Calcola quanto tempo aspettare per il prossimo evento
            // Se è appena passato un tick, aspettiamo fino al prossimo.
            let timeout = self
                .tick_rate
                .checked_sub(self.last_tick.elapsed())
                .unwrap_or(Duration::from_secs(0));

            // 3. Polling degli eventi (tastiera)
            self.handle_events().map_err(|_|"Handle events error")?;

            // 4. Polling della Libreria (Logica)
            // Se sono passati 100ms, chiediamo alla libreria lo snapshot aggiornato
            if self.last_tick.elapsed() >= self.tick_rate {
                // Chiami la tua funzione della libreria
                self.planets = self.orchestrator.get_planet_states();

                // Opzionale: se il gioco deve avanzare, chiami anche l'update
                // self.orchestrator.tick();

                self.last_tick = Instant::now();
            }
        }
        Ok(())
    }

    /// Loop di pausa: consuma solo messaggi UI, tempo fermo
    fn paused_loop(&mut self) -> Result<(), String> {
        // debug_println!("Game is paused. Waiting for resume...");
        self.handle_events().map_err(|_|"Handle events error")?;
        Ok(())
    }
}
// Event Handling
impl App {
    fn handle_events(&mut self) -> Result<(), String> {

        if event::poll(self.tick_rate).map_err(|_| "Polling error")? {
            if let Event::Key(key) = event::read().map_err(|_| "Reading events error")? {
                match (key.code, self.get_game_state()){
                    (KeyCode::Char('q'), _) => {
                        self.exit = true;
                    }
                    (KeyCode::Char('s'), GameState::WaitingStart) => {
                        self.set_game_state(GameState::Running);
                        self.orchestrator.start_all()?;
                    }
                    (KeyCode::Char('p'), GameState::Running) => {
                        self.set_game_state(GameState::Paused);
                    }
                    (KeyCode::Char('p'), GameState::Paused) => {
                        self.set_game_state(GameState::Running);
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }
}

// UI Rendering
impl App {
    fn render_ui(&self, frame: &mut Frame) {
        // Main Layout: 3 columns. 1.Planets 2.1 Explorers 2.2 Instructions 3.Debug Messages
        let main_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Percentage(40),
                Constraint::Percentage(30),
            ])
            .split(frame.area());

        // Planets Area
        let left_layout = main_layout[0];

        // Up Explorers and under Instructions
        let right_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(main_layout[1]);

        // Debug Messages Area (not implemented)
        let debug_area = main_layout[2];

        // 1. RENDERING PLANTS TABLE
        self.render_planets_table(frame, left_layout);

        // 2. RENDERING EXPLORERS (LIST)
        self.render_explorers(frame, right_layout[0]);

        // 3. RENDERING INSTRUCTIONS
        let inst = Paragraph::new("Q: Quit | S: Start | P: Pause").block(
            Block::bordered()
                .title(" Instructions ")
                .border_style(Style::default().fg(Color::DarkGray)),
        );
        frame.render_widget(inst, right_layout[1]);

        // 4. RENDERING DEBUG MESSAGES
        self.render_debug_messages(frame, debug_area);
    }

    fn render_planets_table(&self, frame: &mut Frame, area: Rect) {
        let header = Row::new(vec!["ID", "Rocket", "Energy", "Status", "Incoming"]).style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );

        // TODO discriminate between the number of energy cells
        let rows: Vec<Row> = self
            .planets
            .iter()
            .map(|(id, st)| {
                let energy_str = "■".repeat(0) + &"□".repeat(5);
                // let energy_color = if p.energy == 5 {
                //     Color::Green
                // } else {
                //     Color::White
                // };

                let status = match st {
                    Status::Running => "Running",
                    Status::Paused => "Paused",
                    Status::Dead => "Dead",
                };

                Row::new(vec![
                    Cell::from(id.to_string()),
                    Cell::from("-".to_string()),
                    Cell::from(energy_str),
                    Cell::from(status.to_string()),
                    Cell::from("-".to_string()),
                ])
            })
            .collect();

        let table = Table::new(
            rows,
            [
                Constraint::Min(4),
                Constraint::Min(7),
                Constraint::Min(7),
                Constraint::Min(7),
                Constraint::Min(7),
            ],
        )
        .header(header)
        .block(Block::bordered().title(" One milion crabs galaxy "));

        frame.render_widget(table, area);
    }

    fn render_explorers(&self, frame: &mut Frame, area: Rect) {
        // let items: Vec<ListItem> = self
        //     .explorers
        //     .iter()
        //     .map(|e| {
        //         let mut inv = String::new();
        //         for i in 0..5 {
        //             inv.push_str(if i < e.inventory.len() { "[X]" } else { "[ ]" });
        //         }
        //         ListItem::new(format!(
        //             "Explorer {} @ {} | Inventory: {}",
        //             e.id, e.pos, inv
        //         ))
        //     })
        //     .collect();

        let list = Paragraph::new("Explorers").block(Block::bordered().title(" Explorers Status "));
        frame.render_widget(list, area);
    }

    fn render_debug_messages(&self, frame: &mut Frame, area: Rect) {
        let debug_paragraph = Paragraph::new("Debug messages will appear here.").block(
            Block::bordered()
                .title(" Debug Messages ")
                .border_style(Style::default().fg(Color::DarkGray)),
        );
        frame.render_widget(debug_paragraph, area);
    }
}

fn main() -> Result<(), String> {
    //Init terminal
    let mut terminal = ratatui::init();

    // REVIEW this functionality
    // let sequence = "AAAAAAA".to_string();
    // settings::set_sunray_asteroid_sequence(sequence);
    // settings::set_sunray_asteroid_sequence("AAAAAAASSS".to_string());
    // let sequence = settings::pop_sunray_asteroid_sequence();

    //Init orchestrator
    let orchestrator = Orchestrator::new()?;

    // Create and run game loop
    let mut app = App::new(orchestrator);

    // Initialize by file
    app.inititialize_by_file()?;

    // Startt the app
    let result = app
        .run(&mut terminal)
        .map_err(|_| "An error has occured in the terminal".to_string());

    // Restore terminal
    ratatui::restore();

    // Return possible error from the app run
    result
}
