use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Paragraph},
};

use crate::app::App;

pub fn render_explorers(app: &App, frame: &mut Frame, area: Rect) {
    // let items: Vec<ListItem> = app
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
