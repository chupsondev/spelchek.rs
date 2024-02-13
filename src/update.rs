use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::app_state::AppState;
use crate::prelude::*;

pub fn update(app: &mut AppState) -> Result<()> {
    let key_event = get_key_event()?;

    if let Some(event) = key_event {
        if quit(&event) {
            app.quit();
        }
    }

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
        KeyCode::Char(c) if (c == 'q' || c == 'Q') && key_event.modifiers.is_empty() => {
            true
        }
        KeyCode::Char('c') if key_event.modifiers == KeyModifiers::CONTROL => true,
        KeyCode::Char('d') if key_event.modifiers == KeyModifiers::CONTROL => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quit_check() {
        assert!(!quit(&KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE)));
        assert!(!quit(&KeyEvent::new(KeyCode::Char('y'), KeyModifiers::NONE)));
        assert!(quit(&KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE)));
        assert!(!quit(&KeyEvent::new(KeyCode::Char('q'), KeyModifiers::CONTROL)));
        assert!(quit(&KeyEvent::new(KeyCode::Char('Q'), KeyModifiers::NONE)));
        assert!(quit(&KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL)));
        assert!(!quit(&KeyEvent::new(KeyCode::Char('c'), KeyModifiers::NONE)));
        assert!(quit(&KeyEvent::new(KeyCode::Char('d'), KeyModifiers::CONTROL)));
        assert!(!quit(&KeyEvent::new(KeyCode::Char('d'), KeyModifiers::SHIFT)));
    }
}
