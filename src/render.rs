use crate::app_state::AppState;

use ratatui::style::Color;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::Style;
use ratatui::widgets::{Block, Borders, List, Paragraph};
use ratatui::Frame;

pub fn render(frame: &mut Frame, app: &mut AppState) {
    let layout_fields = create_layout().split(frame.size());

    frame.render_widget(
        Paragraph::new(app.get_buffer().clone())
            .block(Block::new().title("Text").borders(Borders::ALL)),
        layout_fields[1],
    );

    frame.render_widget(
        app.misspellings()
            .iter()
            .map(|f| f.get_word().clone())
            .collect::<List>()
            .block(Block::new().title("Misspellings").borders(Borders::ALL))
            .highlight_style(Style::default().fg(Color::Blue)),
        layout_fields[0],
    );

    frame.render_widget(
        Block::new()
            .title("Correction suggestions")
            .borders(Borders::ALL),
        layout_fields[2],
    );
}

fn create_layout() -> Layout {
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20), // misspelled words list
            Constraint::Percentage(60), // spellchecked text (buffer)
            Constraint::Percentage(20), // spelling suggestions
        ])
}
