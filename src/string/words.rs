use regex::Regex;

/// Split the input string into an array of its words.
///
/// If the regex for filtering purposes is not provided, we will take only groups of normal letters as words by default.
///
/// # Arguments
///
/// * `s` - The input string to split into words.
/// * `re` - Optional regex for word filtering. If provided, only matching substrings are considered words.
///
/// # Returns
///
/// Returns a vector containing the words extracted from the input string.
/// If the input string is empty or None, returns an empty vector.
///
/// # Example
///
/// ```rust
/// use lorust::words;
///
/// let s = "Lorem ipsum dolor sit amet, consectetur adipiscing elit.".to_string();
/// let words = words(s, None);
/// assert_eq!(words, vec!["Lorem", "ipsum", "dolor", "sit", "amet", "consectetur", "adipiscing", "elit"]);
/// ```
pub fn words(s: String, re: Option<&str>) -> Vec<String> {
    match re {
        Some(pattern) => {
            let regex = Regex::new(pattern).unwrap();
            regex
                .find_iter(&s)
                .map(|m| m.as_str().to_string())
                .collect()
        }
        None => {
            let regex = Regex::new(r"\b\w+\b").unwrap();
            regex
                .find_iter(&s)
                .map(|m| m.as_str().to_string())
                .collect()
        }
    }
}
