mod debug;
mod explorers;
mod planets;

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::App;

pub(crate) fn render_game_ui(app: &App, frame: &mut Frame) {
    // Outer Layout: 3 rows. 1.Top Margin 2.Main UI 3.Bottom Margin
    // 1. Globals variables regarding the galaxy
    // 2. Main layout
    // 3. Command line for user debug/gameplay ????
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    // Main Layout: 3 columns. 1.Planets 2.1 Explorers 2.2 Instructions 3.Debug Messages
    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(40),
            Constraint::Percentage(30),
        ])
        .split(outer_layout[1]);

    // Planets Area
    let left_layout = main_layout[0];

    // Up Explorers and under Instructions
    let right_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_layout[1]);

    // Debug Messages Area
    let debug_area = main_layout[2];

    /////////// RENDERING SECTIONS //////////////////
    // Top varaibles rendering and buttons
    render_globals_info(app, frame, outer_layout[0]);

    // 1. RENDERING PLANETS TABLE
    planets::render_planets_table(app, frame, left_layout);

    // 2. RENDERING EXPLORERS (LIST)
    explorers::render_explorers(app, frame, right_layout[0]);

    // 3. RENDERING INSTRUCTIONS
    let inst = Paragraph::new("Q: Quit | S: Start | P: Pause").block(
        Block::bordered()
            .title(" Instructions ")
            .border_style(Style::default().fg(Color::DarkGray)),
    );
    frame.render_widget(inst, right_layout[1]);

    // 4. RENDERING DEBUG MESSAGES
    debug::render_debug_messages(app, frame, debug_area);
}

fn render_globals_info(app: &App, frame: &mut Frame, area: Rect) {
    let title_text = vec![Line::from(vec![
        Span::styled("Simulation Time: ", Style::default().fg(Color::Gray)),
        Span::styled(
            format!("App simultation time???",),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" | ", Style::default().fg(Color::Gray)),
        Span::styled("Total Planets: ", Style::default().fg(Color::Gray)),
        Span::styled(
            format!("{}", app.planets_info.len()),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" | ", Style::default().fg(Color::Gray)),
        Span::styled("Total Explorers: ", Style::default().fg(Color::Gray)),
        Span::styled(
            format!("{}", app.explorers.len()),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" | ", Style::default().fg(Color::Gray)),
        Span::styled("Probability Sunray: ", Style::default().fg(Color::Gray)),
        Span::styled(
            format!("{}%", app.probability_sunray),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
    ])];
    let title = Paragraph::new(title_text).alignment(Alignment::Left).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Blue))
            .style(Style::default().bg(Color::Black)),
    );
    frame.render_widget(title, area);
}
