pub mod algorithm;
use crate::prelude::*;
use std::fs;

/// The representation of a misspelling in the text. The start and end represent the positions in
/// the main buffer at which the word starts and ends.
pub struct Misspelling {
    word: String,
    start: usize,
    end: usize,
    suggestions: Vec<String>,
}

impl Misspelling {
    pub fn new(word: String, start: usize, end: usize) -> Self {
        Misspelling {
            word,
            start,
            end,
            suggestions: Vec::new(),
        }
    }

    pub fn from_range(word: String, range: (usize, usize)) -> Self {
        Self {
            word,
            start: range.0,
            end: range.1,
            suggestions: Vec::new(),
        }
    }

    pub fn get_word(&self) -> &String {
        &self.word
    }
    pub fn get_start(&self) -> usize {
        self.start
    }
    pub fn get_end(&self) -> usize {
        self.end
    }
    pub fn get_range(&self) -> (usize, usize) {
        (self.start, self.end)
    }
    pub fn get_suggestions(&self) -> &Vec<String> {
        &self.suggestions
    }
}

#[derive(Default)]
pub struct Spellchecker {
    dict: Vec<String>,
    misspellings: Vec<Misspelling>,
}

impl Spellchecker {
    pub fn new() -> Result<Self> {
        let dict_path = crate::get_program_files_path().join("dict.txt");

        let dict_content = fs::read(dict_path)?;
        let dict: Vec<String> = String::from_utf8_lossy(&dict_content)
            .into_owned()
            .lines()
            .map(|word| word.trim().to_string())
            .collect();

        Ok(Spellchecker {
            dict,
            misspellings: Vec::new(),
        })
    }

    pub fn set_dict(&mut self, dict: Vec<String>) {
        self.dict = dict.clone();
    }

    pub fn check(&mut self, buffer: &str) {
        let mut word_buf: String = String::new();
        let mut start_pos: usize = 0;
        for (i, c) in buffer.chars().enumerate() {
            if !c.is_alphabetic() {
                if !word_buf.is_empty() {
                    self.check_word_and_add(&word_buf, (start_pos, i));
                }

                word_buf.clear();
                start_pos = i + 1;
            } else {
                word_buf.push(c);
            }
        }
    }

    fn check_word_and_add(&mut self, word: &str, range: (usize, usize)) {
        if algorithm::is_word_correct(word, &self.dict) {
            return;
        }

        self.misspellings
            .push(Misspelling::from_range(word.to_string(), range));
    }

    pub fn misspellings(&mut self) -> &mut Vec<Misspelling> {
        &mut self.misspellings
    }
}
