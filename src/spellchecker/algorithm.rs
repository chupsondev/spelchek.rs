use std::cmp::min;

pub fn search_for_word(word: &str, dict: &[String]) -> Option<usize> {
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

pub fn is_word_correct(word: &str, dict: &[String]) -> bool {
    if word.contains(' ') || word.is_empty() {
        return false;
    }

    search_for_word(word, dict).is_some()
}

pub fn edit_distance(source: &str, target: &str) -> i32 {
    let mut dp: Vec<Vec<i32>> = Vec::new();
    dp.resize_with(source.len() + 1, || vec![0; target.len() + 1]);

    let source: Vec<char> = source.chars().collect();
    let target: Vec<char> = target.chars().collect();

    // source of "" can be made into every substring of target by insertion of another letter
    for i in 0..=target.len() {
        dp[0][i] = i as i32;
    }

    // target of "" can be be made by removing an appropriately larger and larger number of letters
    // from the source
    #[allow(clippy::needless_range_loop)]
    for i in 0..=source.len() {
        dp[i][0] = i as i32;
    }

    for i in 1..=source.len() {
        for j in 1..=target.len() {
            if source[i - 1] == target[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
                continue;
            }

            dp[i][j] = min(
                dp[i - 1][j] + 1, // delete a letter
                min(
                    dp[i][j - 1] + 1,     // insert a character
                    dp[i - 1][j - 1] + 1, // substitute
                ),
            );
        }
    }

    dp[source.len()][target.len()]
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

    #[test]
    fn test_distance_empty_source() {
        assert_eq!(edit_distance("", "elevator"), 8);
    }

    #[test]
    fn test_distance_empty_target() {
        assert_eq!(edit_distance("elevator", ""), 8);
    }

    #[test]
    fn test_distance_only_insert() {
        assert_eq!(edit_distance("elev", "elevator"), 4);
        assert_eq!(edit_distance("a", "abcdef"), 5);
        assert_eq!(edit_distance("a", "aaaaaaaaaaaaaaaaaaaaa"), 20);
    }

    #[test]
    fn test_0_distance() {
        assert_eq!(edit_distance("abc", "abc"), 0);
        assert_eq!(edit_distance("", ""), 0);
        assert_eq!(edit_distance("abcabcabc", "abcabcabc"), 0);
        assert_eq!(edit_distance("abcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabc", "abcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabc"), 0);
    }

    #[test]
    fn test_distance_only_sub() {
        assert_eq!(edit_distance("abc", "def"), 3);
        assert_eq!(edit_distance("kitten", "kityen"), 1);
        assert_eq!(edit_distance("milosz", "mayoss"), 3);
    }

    #[test]
    fn test_edit_distance() {
        assert_eq!(edit_distance("kitten", "smitten"), 2);
        assert_eq!(edit_distance("ths", "this"), 1);
        assert_eq!(edit_distance("bat", "bed"), 2);
        assert_eq!(edit_distance("hello", "kelm"), 3);
        assert_eq!(edit_distance("sittmg", "setting"), 3);
    }
}
