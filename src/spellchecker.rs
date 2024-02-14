pub mod algorithm;
use crate::prelude::*;
use std::fs;

/// The representation of a misspelling in the text. The start and end represent the positions in
/// the main buffer at which the word starts and ends.
#[derive(Debug, PartialEq, Clone)]
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

    pub fn check(&mut self, buffer: &str) {
        let mut word_buf: String = String::new();
        let mut is_proper_word: bool = true;
        let mut start_pos: usize = 0;
        for (i, c) in buffer.chars().enumerate() {
            if c == ' ' || c == '\t' || c == '\n' {
                if !word_buf.is_empty() && is_proper_word {
                    self.check_word_and_add(&word_buf, (start_pos, i - 1));
                }

                word_buf.clear();
                start_pos = i + 1;
                is_proper_word = true;
            } else {
                word_buf.push(c);
                is_proper_word = is_proper_word && c.is_alphabetic();
            }
        }

        if !word_buf.is_empty() && is_proper_word {
            self.check_word_and_add(&word_buf, (start_pos, buffer.len() - 1));
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

#[cfg(test)]
mod tests {
    use super::*;

    fn get_spellchecker() -> Spellchecker {
        Spellchecker::new().unwrap()
    }

    #[test]
    fn test_misspellings_detection() {
        let text = "Ths word aple yelow soem . ? ;";
        let mut spellchecker = get_spellchecker();
        spellchecker.check(text);
        let misspellings = spellchecker.misspellings();
        assert_eq!(misspellings.len(), 4);
    }

    #[test]
    fn test_spellchecking_with_possible_edge_cases() {
        let mut spellchecker = get_spellchecker();

        let text = "THS WORD APLE YELOW";
        spellchecker.check(text);
        assert_eq!(spellchecker.misspellings.len(), 3);
        spellchecker.misspellings.clear();

        let text = "ths this";
        spellchecker.check(text);
        assert_eq!(spellchecker.misspellings.len(), 1);
        spellchecker.misspellings.clear();

        let text = "....................a. 12dadf apple3 aple3";
        spellchecker.check(text);
        assert_eq!(spellchecker.misspellings.len(), 0);
        spellchecker.misspellings.clear();

        let text = ".mis";
        spellchecker.check(text);
        assert_eq!(spellchecker.misspellings.len(), 0);
        spellchecker.misspellings.clear();

        let text = ".miss";
        spellchecker.check(text);
        assert_eq!(spellchecker.misspellings.len(), 0);
        spellchecker.misspellings.clear();

        let text = "miss.";
        spellchecker.check(text);
        assert_eq!(spellchecker.misspellings.len(), 0);
        spellchecker.misspellings.clear();
    }

    #[test]
    fn test_misspelling_position() {
        let mut spellchecker = get_spellchecker();

        let text = "mispeled word wor ";
        spellchecker.check(text);
        assert_eq!(spellchecker.misspellings[0].get_range(), (0, 7));
        assert_eq!(spellchecker.misspellings[1].get_range(), (14, 16));
    }

    #[test]
    fn test_misspelling_position_at_end() {
        let mut spellchecker = get_spellchecker();

        let text = "mispeled";
        spellchecker.check(text);
        assert_eq!(spellchecker.misspellings[0].get_range(), (0, 7));
        spellchecker.misspellings.clear();

        let text = "     mispeled";
        spellchecker.check(text);
        assert_eq!(spellchecker.misspellings[0].get_range(), (5, 12));
    }

    #[test]
    fn test_misspelling() {
        let mut spellchecker = get_spellchecker();

        let text = "mispeled";
        spellchecker.check(text);
        let misspelling = spellchecker.misspellings()[0].clone();
        assert_eq!(
            misspelling,
            Misspelling {
                word: "mispeled".to_string(),
                start: 0,
                end: 7,
                suggestions: Vec::new()
            }
        );
    }

    #[test]
    fn test_misspelling_case() {
        let mut spellchecker = get_spellchecker();

        let text = "MiSpELed";
        spellchecker.check(text);
        let misspelling = spellchecker.misspellings()[0].clone();
        assert_eq!(
            misspelling,
            Misspelling {
                word: "MiSpELed".to_string(),
                start: 0,
                end: 7,
                suggestions: Vec::new()
            }
        );
    }
}
