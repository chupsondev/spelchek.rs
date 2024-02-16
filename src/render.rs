use crate::app_state::AppState;
use crate::spellchecker::Misspelling;

use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::Color;
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

    let misspelling_list = create_misspelling_list_widget(app.spellchecker.misspellings());

    frame.render_stateful_widget(
        misspelling_list,
        layout_fields[0],
        &mut app.misspellings_list_state,
    );

    let suggestions_block = Block::new()
        .title(format!(
            "Suggestions for \"{}\"",
            app.get_misspelled_word().unwrap_or(String::from(""))
        ))
        .borders(Borders::ALL);

    let suggestions = app.get_suggestions().unwrap_or(&Vec::new()).clone();

    frame.render_widget(
        suggestions
            .into_iter()
            .collect::<List>()
            .block(suggestions_block)
            .highlight_style(Style::default().fg(Color::Blue)),
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

fn create_misspelling_list_widget(v: &[Misspelling]) -> List<'_> {
    v.iter()
        .map(|f| f.get_word().clone())
        .collect::<List>()
        .block(Block::new().title("Misspellings").borders(Borders::ALL))
        .highlight_style(Style::default().fg(Color::Blue))
}
