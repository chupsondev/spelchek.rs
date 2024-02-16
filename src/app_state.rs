use ratatui::widgets::ListState;

use crate::prelude::*;
use crate::spellchecker::Spellchecker;
use std::usize;
use std::{fs, fs::canonicalize, path::PathBuf};

pub struct AppState {
    file_path: PathBuf,
    file_buffer: String,
    quit_flag: bool,
    selected_misspelling: Option<usize>,
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

    /// Checks if the currently selected misspelling is in bounds, if not, wraps around
    fn selected_misspelling_inbound(&mut self, misspellings_count: usize) {
        if !self.is_misspelling_selected() {
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
}
