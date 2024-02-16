pub mod algorithm;
use priority_queue::DoublePriorityQueue;

use crate::prelude::*;
use std::{cmp::Ordering, fs};

use self::algorithm::edit_distance;

const NUMBER_OF_SUGGESTIONS: usize = 10;

#[derive(Eq, Debug)]
pub struct SuggestionPriority {
    edit_distance: i32,
    popularity: i64,
}

impl Ord for SuggestionPriority {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.edit_distance.cmp(&other.edit_distance) {
            Ordering::Equal => self.popularity.cmp(&other.popularity),
            Ordering::Greater => Ordering::Less,
            Ordering::Less => Ordering::Greater,
        }
    }
}

impl PartialOrd for SuggestionPriority {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for SuggestionPriority {
    fn eq(&self, other: &Self) -> bool {
        self.edit_distance == other.edit_distance && self.popularity == other.popularity
    }
}

impl SuggestionPriority {
    fn new(edit_distance: i32, popularity: i64) -> Self {
        Self {
            edit_distance,
            popularity,
        }
    }
}

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

    pub fn suggest(&mut self, dict: &Vec<String>) -> &Vec<String> {
        let mut top_suggestions = DoublePriorityQueue::new();

        for entry in dict {
            let mut entry = entry.split_ascii_whitespace();
            let word: &str = entry.next().unwrap();
            let popularity: i64 = entry.next().unwrap().trim().parse().unwrap();

            // yes, I know this is terrible, I'll work on that
            let dist = edit_distance(&self.word.to_lowercase(), word);
            top_suggestions.push(word, SuggestionPriority::new(dist, popularity));
            while top_suggestions.len() > NUMBER_OF_SUGGESTIONS {
                top_suggestions.pop_min();
            }
        }

        self.suggestions = top_suggestions
            .into_sorted_iter()
            .map(|x| x.0.to_owned())
            .rev()
            .collect();
        &self.suggestions
    }
}

#[derive(Default)]
pub struct Spellchecker {
    dict: Vec<String>,
    suggestion_dict: Vec<String>,
    misspellings: Vec<Misspelling>,
}

impl Spellchecker {
    fn load_dict(name: &str) -> Result<Vec<String>> {
        let dict_path = crate::get_program_files_path().join(name);

        let dict_content = fs::read(dict_path)?;
        Ok(String::from_utf8_lossy(&dict_content)
            .into_owned()
            .lines()
            .map(|word| word.trim().to_string())
            .collect())
    }

    pub fn new() -> Result<Self> {
        let dict = Spellchecker::load_dict("dict.txt")?;
        let suggestion_dict = Spellchecker::load_dict("suggestion_dict.txt")?;

        Ok(Spellchecker {
            dict,
            suggestion_dict,
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

    pub fn misspellings(&self) -> &Vec<Misspelling> {
        &self.misspellings
    }

    pub fn misspellings_mut(&mut self) -> &mut Vec<Misspelling> {
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
    fn test_spellchecking() {
        let mut spellchecker = get_spellchecker();

        let text = "this is some text with no misspellings.";
        spellchecker.check(text);
        assert_eq!(spellchecker.misspellings.len(), 0);
        spellchecker.misspellings.clear();

        let text = "some other text which is not misspelled. essentially impotent";
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

    #[test]
    fn test_getting_suggestions() {
        let mut misspelling = Misspelling::new("ths".to_owned(), 0, 0);

        let spellchecker = get_spellchecker();

        misspelling.suggest(&spellchecker.suggestion_dict);
        assert!(misspelling.get_suggestions().contains(&"this".to_string()));
        assert!(misspelling.get_suggestions().contains(&"the".to_string()));
        assert!(
            misspelling
                .get_suggestions()
                .iter()
                .position(|elem| elem == &"this".to_string())
                .unwrap()
                <= 2
        );
    }

    #[test]
    fn test_getting_suggestions_different_misspellings() {
        let spellchecker = get_spellchecker();

        let mut misspelling = Misspelling::new("comon".to_owned(), 0, 0);
        misspelling.suggest(&spellchecker.suggestion_dict);
        assert!(
            misspelling
                .get_suggestions()
                .iter()
                .position(|elem| elem == &"common".to_string())
                .unwrap()
                <= 2
        );

        let mut misspelling = Misspelling::new("womn".to_owned(), 0, 0);
        misspelling.suggest(&spellchecker.suggestion_dict);
        assert!(
            misspelling
                .get_suggestions()
                .iter()
                .position(|elem| elem == &"women".to_string())
                .unwrap()
                <= 2
        );

        // https://en.wikipedia.org/wiki/Commonly_misspelled_English_words

        let mut misspelling = Misspelling::new("amatuer".to_owned(), 0, 0);
        misspelling.suggest(&spellchecker.suggestion_dict);
        assert!(
            misspelling
                .get_suggestions()
                .iter()
                .position(|elem| elem == &"amateur".to_string())
                .unwrap()
                <= 2
        );

        let mut misspelling = Misspelling::new("commited".to_owned(), 0, 0);
        misspelling.suggest(&spellchecker.suggestion_dict);
        assert!(
            misspelling
                .get_suggestions()
                .iter()
                .position(|elem| elem == &"committed".to_string())
                .unwrap()
                <= 2
        );

        let mut misspelling = Misspelling::new("millenium".to_owned(), 0, 0);
        misspelling.suggest(&spellchecker.suggestion_dict);
        assert!(
            misspelling
                .get_suggestions()
                .iter()
                .position(|elem| elem == &"millennium".to_string())
                .unwrap()
                <= 2
        );

        let mut misspelling = Misspelling::new("nieghbor".to_owned(), 0, 0);
        misspelling.suggest(&spellchecker.suggestion_dict);
        assert!(
            misspelling
                .get_suggestions()
                .iter()
                .position(|elem| elem == &"neighbor".to_string())
                .unwrap()
                <= 2
        );
    }

    #[test]
    fn test_suggestion_priority() {
        let mut priorities = vec![
            SuggestionPriority::new(1, i64::MAX),
            SuggestionPriority::new(2, 10),
            SuggestionPriority::new(2, 0),
            SuggestionPriority::new(1, 10),
            SuggestionPriority::new(0, 0),
        ];

        priorities.sort();
        let mut iter = priorities.into_iter().rev();

        assert_eq!(iter.next(), Some(SuggestionPriority::new(0, 0)));
        assert_eq!(iter.next(), Some(SuggestionPriority::new(1, i64::MAX)));
        assert_eq!(iter.next(), Some(SuggestionPriority::new(1, 10)));
        assert_eq!(iter.next(), Some(SuggestionPriority::new(2, 10)));
        assert_eq!(iter.next(), Some(SuggestionPriority::new(2, 0)));
    }
}
