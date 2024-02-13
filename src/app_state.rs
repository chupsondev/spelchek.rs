use crate::prelude::*;
use std::{fs, fs::canonicalize, path::PathBuf};

pub struct AppState {
    file_path: PathBuf,
    file_buffer: String,
    quit_flag: bool,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            file_path: PathBuf::new(),
            file_buffer: String::new(),
            quit_flag: false,
        }
    }
}

impl AppState {
    pub fn new(file_path: PathBuf, file_buffer: String) -> Self {
        let file_path = canonicalize(file_path).unwrap(); // make sure that it's the full path
        Self {
            file_path,
            file_buffer,
            quit_flag: false,
        }
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
}
