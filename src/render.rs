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
        create_spellchecked_text(
            app.get_buffer(),
            app.spellchecker.misspellings(),
            app.selected_misspelling,
        )
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

/// Determines whether a position is the start of some Misspelling. If it is, returns the index of
/// that misspelling in the passed Misspelling slice (&[Misspelling])
fn starts_misspelling(idx: usize, misspellings: &[Misspelling]) -> Option<usize> {
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
        Some(left)
    } else {
        None
    }
}

/// Creates a span representing a Misspelling. If `highlight` is `true`, the Misspelling is also
/// highlighted (has a background).
fn create_misspelling_span(text: &str, highlight: bool) -> Span {
    let style: Style = match highlight {
        false => Style::new()
            .underline_color(Color::LightRed)
            .add_modifier(Modifier::UNDERLINED),
        true => Style::new().bg(Color::Blue),
    };

    Span::styled(text, style)
}

/// Creates a ratatui Paragraph from the text buffer with misspellings underlined and the
/// Misspelling under the index passed in as `highlight_misspelling_index` highlighted.
fn create_spellchecked_text<'a>(
    buf: &'a str,
    misspellings: &'a [Misspelling],
    highlight_misspelling_index: Option<usize>,
) -> Paragraph<'a> {
    let mut lines: Vec<Line> = Vec::new();

    let mut current_line_spans: Vec<Span> = Vec::new(); // The spans on the currently processed line
    let mut span_start: usize = 0; // The index of the character on which the span to be added next
                                   // starts, or the one after the previously added span
    for (i, c) in buf.chars().enumerate() {
        // If the current index is inside some misspelling, add it as a span.
        if let Some(misspelling_idx) = starts_misspelling(i, misspellings) {
            let misspelling: &Misspelling = &misspellings[misspelling_idx];
            current_line_spans.push(Span::raw(&buf[span_start..i]));

            // Add misspelling span to the current line spans vector
            current_line_spans.push(create_misspelling_span(
                &buf[misspelling.get_start()..=misspelling.get_end()],
                Some(misspelling_idx) == highlight_misspelling_index,
            ));

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
        assert_eq!(starts_misspelling(0, &misspellings), Some(0));
        assert_eq!(starts_misspelling(10, &misspellings), Some(1));
        assert_eq!(starts_misspelling(30, &misspellings), Some(2));
        assert_eq!(starts_misspelling(31, &misspellings), None);
        assert_eq!(starts_misspelling(15, &misspellings), None);
        assert_eq!(starts_misspelling(20, &misspellings), None);
    }

    #[test]
    fn test_creating_text_paragraph_no_misspellings() {
        let text = "Some example text with no misspellings";
        let misspellings = Vec::new();
        assert_eq!(
            create_spellchecked_text(text, &misspellings, None),
            Paragraph::new(Line::from(vec![Span::raw(text)]))
        );
        // With multiple lines
        let text = "Some text with\nnew line characters\n";
        assert_eq!(
            create_spellchecked_text(text, &misspellings, None),
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
            create_spellchecked_text(text, &misspellings, None),
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

    #[test]
    // Tests creating the text paragraph with one of the Misspellings highlighted
    fn test_creating_text_paragraph_with_highlight() {
        let text = "Sme text with sme misspellings.\nFinished by a mispeling";
        let misspellings = vec![
            Misspelling::new(String::from("Sme"), 0, 2),
            Misspelling::new(String::from("sme"), 14, 16),
            Misspelling::new(String::from("mispeling"), 46, 54),
        ];
        assert_eq!(
            create_spellchecked_text(text, &misspellings, Some(0)),
            Paragraph::new(vec![
                Line::from(vec![
                    Span::raw(""),
                    Span::styled("Sme", Style::new().bg(Color::Blue)),
                    Span::raw(" text with "),
                    miss_span("sme"),
                    Span::raw(" misspellings.\n")
                ]),
                Line::from(vec![Span::raw("Finished by a "), miss_span("mispeling")])
            ])
        );
    }

    #[test]
    fn test_create_misspelling() {
        assert_eq!(
            create_misspelling_span("hello world", false),
            Span::styled(
                "hello world",
                Style::new()
                    .underline_color(Color::LightRed)
                    .add_modifier(Modifier::UNDERLINED)
            )
        );

        assert_eq!(
            create_misspelling_span("hello world", true),
            Span::styled("hello world", Style::new().bg(Color::Blue))
        );
    }
}
