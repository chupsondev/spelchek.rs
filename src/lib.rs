// Define modules
pub mod app_state; // holds the app state
pub mod render; // responsible for rendering the ratatui tui
pub mod update; // updates every frame, handles input

// the actual logic behind spellchecking
pub mod spellchecker; // the main module controlling spellchecking // the algorithms for calculating word distance and similar

pub mod prelude; // global exports and other
use crate::prelude::*;

use crate::app_state::AppState;

use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use std::io::{self, Write};

use std::fs::{self, canonicalize};
use std::panic;
use std::path::PathBuf;

pub struct Config {
    spellchecked_file_path: PathBuf,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self> {
        let requested_file_path = match args.get(0) {
            Some(arg) => arg,
            None => {
                return Err(anyhow::anyhow!("not enough arguments given"));
            }
        };

        let requested_file_path = canonicalize(requested_file_path)?;

        Ok(Self {
            spellchecked_file_path: requested_file_path,
        })
    }

    pub fn get_spellchecked_file_path(&self) -> &PathBuf {
        &self.spellchecked_file_path
    }
}

pub fn run(config: &Config) -> Result<()> {
    let mut terminal = start_terminal()?;

    initialize_panic_hook();

    let path = config.get_spellchecked_file_path().clone();
    let file_contents: String = String::from_utf8_lossy(&fs::read(&path)?).to_string();
    let mut app = AppState::new(path, file_contents);

    while !app.should_quit() {
        terminal.draw(|frame| render::render(frame, &mut app))?;
        update::update(&mut app)?;
    }

    close_terminal()?;
    terminal.show_cursor()?;
    Ok(())
}

fn start_terminal() -> Result<Terminal<CrosstermBackend<impl Write>>> {
    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
    terminal.clear()?;

    Ok(terminal)
}

fn close_terminal() -> Result<()> {
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}

fn initialize_panic_hook() {
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        close_terminal().unwrap();
        original_hook(panic_info);
    }))
}
