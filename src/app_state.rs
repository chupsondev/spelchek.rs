use anyhow::anyhow;
use ratatui::widgets::ListState;

use crate::prelude::*;
use crate::spellchecker::{Misspelling, Spellchecker};
use std::fs::File;
use std::io::Write;
use std::{fs, fs::canonicalize, path::PathBuf};
use std::{u8, usize};

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
        let mut suggestion: String = selected_misspelling
            .get_suggestions()
            .get(self.selected_suggestion.unwrap())
            .unwrap()
            .to_string();
        match_case(selected_misspelling.get_word(), &mut suggestion); // Match the case of the
                                                                      // corrected word to the previously misspelled word

        let misspelling_len: usize =
            selected_misspelling.get_end() - selected_misspelling.get_start() + 1;

        let len_delta: i32 = suggestion.len() as i32 - misspelling_len as i32; // The difference in length between
                                                                               // the previously misspelled word and the new correction

        // Splits off the part of the buffer containing all text from the beginning of the
        // misspelling to the end of the buffer, and stores it.
        let misspelling_start: usize = selected_misspelling.get_start();
        let buffer_after: String = self.file_buffer.split_off(misspelling_start);
        self.file_buffer.push_str(&suggestion); // Adds the suggestion to the end of the buffer
        self.file_buffer.push_str(&buffer_after[misspelling_len..]); // Adds the
                                                                     // rest of the text to the end of the buffer

        self.spellchecker
            .offset_misspelling_positions(len_delta, selected_misspelling_idx);

        // The number of misspellings is changed, therefore the selected misspelling must be
        // updated.
        self.selected_misspelling_inbound(self.spellchecker.misspellings.len());
        self.set_misspellings_list_state();
    }

    /// Saves the corrected texts by replacing the file contents with the contents of the buffer.
    pub fn save_file(&self) -> Result<()> {
        if !self.file_path.exists() {
            return Err(anyhow!("opened file doesn't exist and can't be written to"));
        }

        let mut file = File::create(self.file_path.clone())?;

        let buf_bytes: Vec<u8> = self.file_buffer.bytes().collect();
        file.write_all(&buf_bytes)?;
        Ok(())
    }
}

/// Tries to match case of `target` to that of `source`. It does so by matching the case of
/// individual characters. For each index in `source`, if that index also exists in `target` it
/// sets the case of the character on that index in `target` to be the same as the character on that
/// index in `source`.
///
///
/// # Example
/// ```ignore
/// let mut target = String::from("target");
/// match_case("SoUrCe", &mut target);
/// assert_eq!(target, "TaRgEt");
/// ```
// Assumes UTF-8 encoding
fn match_case(source: &str, target: &mut String) {
    let mut new_target: String = String::new();

    let mut source_chars = source.chars();
    for target_char in target.chars() {
        let source_char: char = match source_chars.next() {
            Some(c) => c,

            // If there are no characters of `source` left to match case to, add
            // unchanged character to `new_target`
            None => {
                new_target.push(target_char);
                continue;
            }
        };

        if source_char.is_uppercase() {
            new_target.push_str(&target_char.to_uppercase().to_string());
        } else {
            new_target.push_str(&target_char.to_lowercase().to_string());
        }
    }
    *target = new_target;
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

    #[test]
    fn test_accepting_suggestion_match_case() {
        let text = "HeLllO world, this is some example text.";
        let mut app_state = AppState::new(PathBuf::from("/"), text.to_string()).unwrap();
        app_state.check_spelling();
        app_state.select_first_misspelling();
        app_state.suggest_selected();
        app_state.select_next_suggestion();

        app_state.accept_suggestion();

        assert_eq!(
            app_state.file_buffer,
            "HeLlo world, this is some example text." // Here I take a gamble
                                                      // and assume the top suggestion for "helllo" is "hello". If for some reason it's not,
                                                      // this test will fail. If it's failing, check the suggestions.
        );
    }

    #[test]
    fn test_match_case() {
        let mut target = String::from("hello");
        match_case("World", &mut target);
        assert_eq!(target, "Hello");

        let mut target = String::from("hello");
        match_case("worlD", &mut target);
        assert_eq!(target, "hellO");

        let mut target = String::from("hello");
        match_case("worLd", &mut target);
        assert_eq!(target, "helLo");

        let mut target = String::from("hello");
        match_case("", &mut target);
        assert_eq!(target, "hello");

        let mut target = String::from("");
        match_case("Hello world", &mut target);
        assert_eq!(target, "");

        let mut target = String::from("hello");
        match_case("AntidisestablishmentARIANISM", &mut target);
        assert_eq!(target, "Hello");

        let mut target = String::from("antidisestablishmentarianism");
        match_case("WorlD", &mut target);
        assert_eq!(target, "AntiDisestablishmentarianism");
    }
}
