pub fn search_for_word(word: &str, dict: &Vec<String>) -> Option<usize> {
    if word.contains(' ') || word.is_empty() {
        return None;
    }

    let word = word.to_lowercase();

    let mut left: usize = 0;
    let mut right: usize = dict.len() - 1;
    let mut middle: usize;

    let word = &word.to_string();

    while left != right {
        middle = (left + right + 1) / 2;
        let middle_word = dict.get(middle).unwrap();

        if middle_word > word {
            right = middle - 1;
        } else {
            left = middle;
        }
    }

    if dict.get(left).unwrap() == word {
        Some(left)
    } else if dict.get(right).unwrap() == word {
        Some(right)
    } else {
        None
    }
}

pub fn is_word_correct(word: &str, dict: &Vec<String>) -> bool {
    if word.contains(' ') || word.is_empty() {
        return false;
    }

   search_for_word(word, dict).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_dict() -> Vec<String> {
        ["apple", "apples", "banana", "blue", "cucumber", "yellow"]
            .iter()
            .map(|s| s.to_string())
            .collect()
    }

    #[test]
    fn test_finding_existing_word() {
        let dict = create_dict();
        assert_eq!(search_for_word("apple", &dict), Some(0));
        assert_eq!(search_for_word("blue", &dict), Some(3));
    }

    #[test]
    fn test_finding_nonexistant_word() {
        let dict = create_dict();
        assert_eq!(search_for_word("applee", &dict), None);
        assert_eq!(search_for_word("red", &dict), None);
    }

    #[test]
    fn test_checking_correct_word() {
        let dict = create_dict();
        assert!(is_word_correct("apple", &dict));
        assert!(is_word_correct("apples", &dict));
        assert!(is_word_correct("banana", &dict));
        assert!(is_word_correct("blue", &dict));
        assert!(is_word_correct("cucumber", &dict));
        assert!(is_word_correct("yellow", &dict));
    }

    #[test]
    fn test_checking_incorrect_word() {
        let dict = create_dict();
        assert!(!is_word_correct("kiwi", &dict));
        assert!(!is_word_correct("oranges", &dict));
        assert!(!is_word_correct("dragon", &dict));
        assert!(!is_word_correct("blah", &dict));
        assert!(!is_word_correct("clue", &dict));
        assert!(!is_word_correct("dheubh", &dict));
    }

    #[test]
    fn test_checking_not_word() {
        let dict = create_dict();
        assert!(!is_word_correct("some phrase", &dict));
        assert_eq!(search_for_word("some phrase", &dict), None);
    }

    #[test]
    fn test_case_sensitivity() {
        let dict = create_dict();
        assert!(is_word_correct("BaNAna", &dict));
        assert!(is_word_correct("APPLE", &dict));
        assert!(is_word_correct("yellow", &dict));
    }
}
