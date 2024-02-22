use crate::app_state::AppState;
use crate::spellchecker::Misspelling;

use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::Style;
use ratatui::style::{Color, Modifier};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};
use ratatui::Frame;

pub fn render(frame: &mut Frame, app: &mut AppState) {
    let layout_fields = create_layout().split(frame.size());

    frame.render_widget(
        create_spellchecked_text(app.get_buffer(), app.spellchecker.misspellings())
            .block(Block::new().title("Text").borders(Borders::ALL)),
        layout_fields[1],
    );

    let misspelling_list =
        create_boxed_list_widget(app.spellchecker.misspellings().iter(), "Misspellings");

    frame.render_stateful_widget(
        misspelling_list,
        layout_fields[0],
        &mut app.misspellings_list_state,
    );

    let suggestions = app.get_suggestions().unwrap_or(&Vec::new()).clone();

    let mut state = ListState::default();
    state.select(app.selected_suggestion);
    frame.render_stateful_widget(
        create_boxed_list_widget(
            suggestions.into_iter(),
            &format!(
                "Suggestions for \"{}\"",
                app.get_misspelled_word().unwrap_or(String::from(""))
            ),
        ),
        layout_fields[2],
        &mut state,
    );
}

/// Determines whether a position is the start of some Misspelling. If it is, returns a reference
/// to that Misspelling. Otherwise, returns None.
fn starts_misspelling(idx: usize, misspellings: &[Misspelling]) -> Option<&Misspelling> {
    if misspellings.is_empty() {
        return None;
    }

    let mut left = 0;
    let mut right = misspellings.len() - 1;
    while left != right {
        let mid = (left + right + 1) / 2;
        let misspelling_start_idx = misspellings[mid].get_start();

        if misspelling_start_idx > idx {
            right = mid - 1;
        } else {
            left = mid;
        }
    }
    let misspelling_candidate = &misspellings[left];
    if misspelling_candidate.get_start() == idx {
        Some(misspelling_candidate)
    } else {
        None
    }
}

/// Creates a ratatui Paragraph from the text buffer with misspellings underlined
fn create_spellchecked_text<'a>(buf: &'a str, misspellings: &'a [Misspelling]) -> Paragraph<'a> {
    let mut lines: Vec<Line> = Vec::new();

    let mut current_line_spans: Vec<Span> = Vec::new(); // The spans on the currently processed line
    let mut span_start: usize = 0; // The index of the character on which the span to be added next
                                   // starts, or the one after the previously added span
    for (i, c) in buf.chars().enumerate() {
        // If the current index is inside some misspelling, add it as a span.
        if let Some(misspelling) = starts_misspelling(i, misspellings) {
            current_line_spans.push(Span::raw(&buf[span_start..i]));

            current_line_spans.push(Span::styled(
                &buf[misspelling.get_start()..=misspelling.get_end()],
                Style::new()
                    .underline_color(Color::LightRed)
                    .add_modifier(Modifier::UNDERLINED),
            )); // Add misspelling span to the current line spans vector

            span_start = misspelling.get_end() + 1; // Set the start of the next span to after the
                                                    // end of the misspelling span.
            continue;
        }
        if c == '\n' {
            // If there is some span to add, add it.
            current_line_spans.push(Span::raw(&buf[span_start..=i]));

            lines.push(Line::from(current_line_spans));
            current_line_spans = Vec::new();
            span_start = i + 1;
        }
    }
    if span_start < buf.len() {
        current_line_spans.push(Span::raw(&buf[span_start..buf.len()]));
    }
    if !current_line_spans.is_empty() {
        lines.push(Line::from(current_line_spans));
    }
    Paragraph::new(lines)
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

fn create_boxed_list_widget<'a, T>(v: T, box_title: &'a str) -> List<'a>
where
    T: Iterator,
    T::Item: Into<ListItem<'a>>,
{
    v.collect::<List>()
        .block(Block::new().title(box_title).borders(Borders::ALL))
        .highlight_style(Style::default().fg(Color::Blue))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_starts_misspelling_no_misspellings() {
        assert_eq!(starts_misspelling(10, &Vec::new()), None);
        assert_eq!(starts_misspelling(0, &Vec::new()), None);
        assert_eq!(starts_misspelling(9999999999999999, &Vec::new()), None);
    }

    #[test]
    fn test_starts_misspelling() {
        let misspellings = vec![
            Misspelling::new(String::from("1"), 0, 5),
            Misspelling::new(String::from("2"), 10, 20),
            Misspelling::new(String::from("3"), 30, 33),
        ];
        assert_eq!(starts_misspelling(2, &misspellings), None);
        assert_eq!(starts_misspelling(0, &misspellings), Some(&misspellings[0]));
        assert_eq!(
            starts_misspelling(10, &misspellings),
            Some(&misspellings[1])
        );
        assert_eq!(
            starts_misspelling(30, &misspellings),
            Some(&misspellings[2])
        );
        assert_eq!(starts_misspelling(31, &misspellings), None);
        assert_eq!(starts_misspelling(15, &misspellings), None);
        assert_eq!(starts_misspelling(20, &misspellings), None);
    }

    #[test]
    fn test_creating_text_paragraph_no_misspellings() {
        let text = "Some example text with no misspellings";
        let misspellings = Vec::new();
        assert_eq!(
            create_spellchecked_text(text, &misspellings),
            Paragraph::new(Line::from(vec![Span::raw(text)]))
        );
        // With multiple lines
        let text = "Some text with\nnew line characters\n";
        assert_eq!(
            create_spellchecked_text(text, &misspellings),
            Paragraph::new(vec![
                Line::from(vec![Span::raw("Some text with\n"),]),
                Line::from(vec![Span::raw("new line characters\n")])
            ])
        );
    }

    fn miss_span(s: &str) -> Span {
        Span::styled(
            s,
            Style::new()
                .underline_color(Color::LightRed)
                .add_modifier(Modifier::UNDERLINED),
        )
    }

    #[test]
    fn test_creating_text_paragraph() {
        let text = "Sme text with sme misspellings.\nFinished by a mispeling";
        let misspellings = vec![
            Misspelling::new(String::from("Sme"), 0, 2),
            Misspelling::new(String::from("sme"), 14, 16),
            Misspelling::new(String::from("mispeling"), 46, 54),
        ];
        assert_eq!(
            create_spellchecked_text(text, &misspellings),
            Paragraph::new(vec![
                Line::from(vec![
                    Span::raw(""),
                    miss_span("Sme"),
                    Span::raw(" text with "),
                    miss_span("sme"),
                    Span::raw(" misspellings.\n")
                ]),
                Line::from(vec![Span::raw("Finished by a "), miss_span("mispeling")])
            ])
        );
    }
}
