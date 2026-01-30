use omc_galaxy::Status;
use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Cell, Row, Table},
};

use crate::app::App;

pub fn render_planets_table(app: &App, frame: &mut Frame, area: Rect) {
    let header = Row::new(vec!["ID", "Rocket", "Energy", "Status", "Incoming"]).style(
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    );

    // TODO: discriminate between the number of energy cells
    let rows: Vec<Row> = app
        .planets_info
        .iter()
        .map(|(id, info)| {
            let energy_str = "■".repeat(info.charged_cells_count)
                + &"□".repeat(info.energy_cells.len() - info.charged_cells_count);
            // let energy_color = if p.energy == 5 {
            //     Color::Green
            // } else {
            //     Color::White
            // };

            let status = match info.status {
                Status::Running => "Running",
                Status::Paused => "Paused",
                Status::Dead => "Dead",
            };

            Row::new(vec![
                Cell::from(id.to_string()),
                Cell::from(info.rocket.to_string()),
                Cell::from(energy_str),
                Cell::from(status.to_string()),
                Cell::from("-".to_string()),
            ])
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Length(4),
            Constraint::Min(7),
            Constraint::Min(7),
            Constraint::Min(7),
            Constraint::Min(7),
        ],
    )
    .header(header)
    .block(Block::bordered().title(" One million crabs galaxy "));

    frame.render_widget(table, area);
}
