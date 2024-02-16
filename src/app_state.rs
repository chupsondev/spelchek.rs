use ratatui::widgets::ListState;

use crate::spellchecker::Spellchecker;
use crate::{prelude::*, spellchecker};
use std::usize;
use std::{fs, fs::canonicalize, path::PathBuf};

pub struct AppState {
    file_path: PathBuf,
    file_buffer: String,
    quit_flag: bool,
    selected_misspelling: Option<usize>,
    misspellings_list_state: ListState,
    spellchecker: Spellchecker,
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

    pub fn misspellings_list_state(&mut self) -> &mut ListState {
        &mut self.misspellings_list_state
    }

    pub fn check_spelling(&mut self) {
        self.spellchecker.check(&self.file_buffer);
    }

    pub fn misspellings(&self) -> &Vec<spellchecker::Misspelling> {
        self.spellchecker.misspellings()
    }

    pub fn misspellings_mut(&mut self) -> &mut Vec<spellchecker::Misspelling> {
        self.spellchecker.misspellings_mut()
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
        let count = self.misspellings().len();

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
        let count = self.misspellings().len();

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
}
