use ratatui::widgets::ListState;

use crate::prelude::*;
use crate::spellchecker::{Misspelling, Spellchecker};
use std::usize;
use std::{fs, fs::canonicalize, path::PathBuf};

#[derive(Debug)]
pub struct AppState {
    file_path: PathBuf,
    file_buffer: String,
    quit_flag: bool,
    pub selected_misspelling: Option<usize>,
    pub selected_suggestion: Option<usize>,
    pub misspellings_list_state: ListState,
    pub spellchecker: Spellchecker,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            file_path: PathBuf::new(),
            file_buffer: String::new(),
            quit_flag: false,
            selected_misspelling: None,
            selected_suggestion: None,
            misspellings_list_state: ListState::default(),
            spellchecker: Spellchecker::default(),
        }
    }
}

impl AppState {
    pub fn new(file_path: PathBuf, file_buffer: String) -> Result<Self> {
        let file_path = canonicalize(file_path).unwrap(); // make sure that it's the full path
        Ok(Self {
            file_path,
            file_buffer,
            quit_flag: false,
            selected_misspelling: None,
            selected_suggestion: None,
            misspellings_list_state: ListState::default(),
            spellchecker: Spellchecker::new()?,
        })
    }

    pub fn write_buffer(&self) -> Result<()> {
        fs::write(&self.file_path, &self.file_buffer)?;
        Ok(())
    }

    pub fn get_buffer(&self) -> &String {
        &self.file_buffer
    }

    pub fn should_quit(&self) -> bool {
        self.quit_flag
    }

    pub fn quit(&mut self) {
        self.quit_flag = true;
    }

    pub fn check_spelling(&mut self) {
        self.spellchecker.check(&self.file_buffer);
    }

    fn is_misspelling_selected(&self) -> bool {
        self.selected_misspelling.is_some()
    }

    fn select_first_misspelling(&mut self) {
        self.selected_misspelling = Some(0);
        self.set_misspellings_list_state();
    }

    /// Checks if the currently selected misspelling is in bounds, if not, wraps around. If there
    /// are no misspellings, sets the currently selected to `None`.
    fn selected_misspelling_inbound(&mut self, misspellings_count: usize) {
        if !self.is_misspelling_selected() {
            return;
        }

        if misspellings_count == 0 {
            self.selected_misspelling = None;
            return;
        }

        if self.selected_misspelling == Some(usize::MAX) {
            self.selected_misspelling = Some(misspellings_count - 1);
            return;
        }

        if self.selected_misspelling.unwrap() > misspellings_count - 1 {
            self.selected_misspelling = Some(0);
        }
    }

    fn set_misspellings_list_state(&mut self) {
        self.misspellings_list_state
            .select(self.selected_misspelling);
    }

    pub fn select_next_misspelling(&mut self) {
        let count = self.spellchecker.misspellings().len();

        if count == 0 {
            self.selected_misspelling = None;
            return;
        }

        if !self.is_misspelling_selected() {
            self.select_first_misspelling();
        } else {
            self.selected_misspelling = Some(self.selected_misspelling.unwrap() + 1);
        }

        self.selected_misspelling_inbound(count);
        self.set_misspellings_list_state();
    }

    pub fn select_previous_misspelling(&mut self) {
        let count = self.spellchecker.misspellings().len();

        if count == 0 {
            self.selected_misspelling = None;
            return;
        }

        if !self.is_misspelling_selected() {
            self.select_first_misspelling();
        } else {
            self.selected_misspelling = Some(
                self.selected_misspelling
                    .unwrap()
                    .checked_sub(1)
                    .unwrap_or(usize::MAX),
            );
        }

        self.selected_misspelling_inbound(count);
        self.set_misspellings_list_state();
    }

    /// Returns the reference to the selected misspelling if one is selected, otherwise None
    fn get_selected_misspelling(&self) -> Option<&Misspelling> {
        match self.selected_misspelling {
            None => None,
            Some(idx) => Some(
                self.spellchecker
                    .misspellings()
                    .get(idx)
                    .expect("selected misspelling doesn't exist (something went very wrong)"),
            ),
        }
    }

    /// Selects the next suggestion for correcting the currently selected misspelling. If no
    /// misspelling is selected, it does nothing.
    pub fn select_next_suggestion(&mut self) {
        let selected_misspelling = match self.get_selected_misspelling() {
            None => {
                return;
            }
            Some(misspelling) => misspelling,
        };

        self.selected_suggestion =
            selected_misspelling.get_next_suggestion_index(self.selected_suggestion);
    }

    /// Selects the previous suggestion for correcting the currently selected misspelling. If no
    /// misspelling is selected, it does nothing.
    pub fn select_previous_suggestion(&mut self) {
        let selected_misspelling = match self.get_selected_misspelling() {
            None => {
                return;
            }
            Some(misspelling) => misspelling,
        };

        self.selected_suggestion =
            selected_misspelling.get_previous_suggestion_index(self.selected_suggestion);
    }

    /// Generates suggestions for currently selected misspelling
    pub fn suggest_selected(&mut self) {
        if self.selected_misspelling.is_none() {
            return;
        }

        self.spellchecker
            .suggest(self.selected_misspelling.unwrap());
    }

    /// Get suggestions for the currently selected misspelling
    pub fn get_suggestions(&self) -> Option<&Vec<String>> {
        if !self.is_misspelling_selected() {
            None
        } else {
            Some(
                self.spellchecker
                    .get_suggestions(self.selected_misspelling.unwrap()),
            )
        }
    }

    pub fn get_misspelled_word(&self) -> Option<String> {
        if !self.is_misspelling_selected() {
            return None;
        }

        self.spellchecker
            .misspellings()
            .get(self.selected_misspelling.unwrap())
            .map(|misspelling| misspelling.get_word().clone())
    }

    /// Accepts the currently selected suggestion for the currently selected misspelling.
    pub fn accept_suggestion(&mut self) {
        // If there is no selected misspelling or suggestion, do nothing.
        if self.selected_misspelling.is_none() || self.selected_suggestion.is_none() {
            return;
        }
        let selected_misspelling_idx = self
            .selected_misspelling
            .expect("should always work due to preceding if");

        // Retrieve the selected misspelling and remove it from the list - as it is about to be
        // corrected it will not be a misspelling anymore.
        // NOTE: If something goes wrong with the correction this could be an issue, as it is done
        // before the correction
        let selected_misspelling = self
            .spellchecker
            .misspellings
            .remove(selected_misspelling_idx);

        // The suggestion to be put in place of the misspelled word
        let suggestion: &str = selected_misspelling
            .get_suggestions()
            .get(self.selected_suggestion.unwrap())
            .unwrap();

        let misspelling_len: usize =
            selected_misspelling.get_end() - selected_misspelling.get_start() + 1;

        let len_delta: i32 = suggestion.len() as i32 - misspelling_len as i32; // The difference in length between
                                                                               // the previously misspelled word and the new correction

        // Splits off the part of the buffer containing all text from the beginning of the
        // misspelling to the end of the buffer, and stores it.
        let misspelling_start: usize = selected_misspelling.get_start();
        let buffer_after: String = self.file_buffer.split_off(misspelling_start);
        self.file_buffer.push_str(suggestion); // Adds the suggestion to the end of the buffer
                                               // TODO: Make suggestion try to match the case of the misspelling. Example: if the
                                               // misspelling starts with a capital letter the suggestion should as well.

        self.file_buffer.push_str(&buffer_after[misspelling_len..]); // Adds the
                                                                     // rest of the text to the end of the buffer

        self.spellchecker
            .offset_misspelling_positions(len_delta, selected_misspelling_idx);

        // The number of misspellings is changed, therefore the selected misspelling must be
        // updated.
        self.selected_misspelling_inbound(self.spellchecker.misspellings.len());
        self.set_misspellings_list_state();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accepting_suggestion() {
        let text = "Hello world, thsi is some example text.";
        let mut app_state = AppState::new(PathBuf::from("/"), text.to_string()).unwrap();
        app_state.check_spelling();
        app_state.select_first_misspelling();
        app_state.suggest_selected();
        app_state.select_next_suggestion();

        let suggestion = app_state
            .get_suggestions()
            .unwrap()
            .get(app_state.selected_suggestion.unwrap())
            .unwrap()
            .clone();
        app_state.accept_suggestion();

        assert_eq!(
            app_state.file_buffer,
            format!("Hello world, {} is some example text.", suggestion) // Can't be sure what
                                                                         // suggestion is the first one
        );
    }

    #[test]
    // The corrected misspelling is the last word
    fn test_accepting_suggestion_last_word() {
        let text = "This piece of text ends with a mispeling";
        let mut app_state = AppState::new(PathBuf::from("/"), text.to_string()).unwrap();
        app_state.check_spelling();
        app_state.select_first_misspelling();
        app_state.suggest_selected();
        app_state.select_next_suggestion();

        let suggestion = app_state
            .get_suggestions()
            .unwrap()
            .get(app_state.selected_suggestion.unwrap())
            .unwrap()
            .clone();
        app_state.accept_suggestion();

        assert_eq!(
            app_state.file_buffer,
            format!("This piece of text ends with a {}", suggestion) // Can't be sure what
                                                                     // suggestion is the first one
        );
    }

    #[test]
    fn test_accepting_suggestion_no_misspelling() {
        let text = "Hello world";
        let mut app_state = AppState::new(PathBuf::from("/"), text.to_string()).unwrap();
        app_state.accept_suggestion();
        app_state.accept_suggestion();
        app_state.accept_suggestion();
        app_state.accept_suggestion();
        app_state.accept_suggestion();

        assert_eq!(app_state.file_buffer, "Hello world");
    }
}
