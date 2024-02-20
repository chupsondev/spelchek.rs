use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::app_state::AppState;
use crate::prelude::*;

pub fn update(app: &mut AppState) -> Result<()> {
    app.suggest_selected();

    let key_event = get_key_event()?;

    if key_event.is_none() {
        return Ok(());
    }

    let key_event = key_event.unwrap();

    if quit(&key_event) {
        app.quit();
    }

    misspelling_selection(&key_event, app);
    suggestion_selection(&key_event, app);

    Ok(())
}

fn get_key_event() -> Result<Option<KeyEvent>> {
    if crossterm::event::poll(std::time::Duration::from_millis(200))? {
        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                return Ok(None);
            }
            return Ok(Some(key));
        }
    }
    Ok(None)
}

fn quit(key_event: &KeyEvent) -> bool {
    match key_event.code {
        KeyCode::Char(c) if (c == 'q' || c == 'Q') && key_event.modifiers.is_empty() => true,
        KeyCode::Char('c') if key_event.modifiers == KeyModifiers::CONTROL => true,
        KeyCode::Char('d') if key_event.modifiers == KeyModifiers::CONTROL => true,
        _ => false,
    }
}

fn misspelling_selection(key_event: &KeyEvent, app: &mut AppState) {
    match key_event.code {
        KeyCode::BackTab if key_event.modifiers == KeyModifiers::SHIFT => {
            app.select_previous_misspelling()
        }
        KeyCode::Tab if key_event.modifiers.is_empty() => app.select_next_misspelling(),
        _ => {}
    }
}

fn suggestion_selection(key_event: &KeyEvent, app: &mut AppState) {
    match key_event.code {
        KeyCode::Char('j') => app.select_next_suggestion(),
        KeyCode::Char('k') => app.select_previous_suggestion(),
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quit_check() {
        assert!(!quit(&KeyEvent::new(
            KeyCode::Char('a'),
            KeyModifiers::NONE
        )));
        assert!(!quit(&KeyEvent::new(
            KeyCode::Char('y'),
            KeyModifiers::NONE
        )));
        assert!(quit(&KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE)));
        assert!(!quit(&KeyEvent::new(
            KeyCode::Char('q'),
            KeyModifiers::CONTROL
        )));
        assert!(quit(&KeyEvent::new(KeyCode::Char('Q'), KeyModifiers::NONE)));
        assert!(quit(&KeyEvent::new(
            KeyCode::Char('c'),
            KeyModifiers::CONTROL
        )));
        assert!(!quit(&KeyEvent::new(
            KeyCode::Char('c'),
            KeyModifiers::NONE
        )));
        assert!(quit(&KeyEvent::new(
            KeyCode::Char('d'),
            KeyModifiers::CONTROL
        )));
        assert!(!quit(&KeyEvent::new(
            KeyCode::Char('d'),
            KeyModifiers::SHIFT
        )));
    }
}
