use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Paragraph},
};

use crate::app::App;

pub fn render_debug_messages(app: &App, frame: &mut Frame, area: Rect) {
    let debug_paragraph = Paragraph::new("Debug messages will appear here.").block(
        Block::bordered()
            .title(" Debug Messages ")
            .border_style(Style::default().fg(Color::DarkGray)),
    );
    frame.render_widget(debug_paragraph, area);
}
