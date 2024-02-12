// Define modules
pub mod app_state; // holds the app state
pub mod render; // responsible for rendering the ratatui tui
pub mod update; // updates every frame, handles input

// the actual logic behind spellchecking
pub mod spellchecker; // the main module controlling spellchecking // the algorithms for calculating word distance and similar

pub mod prelude; // global exports and other
use crate::prelude::*;
