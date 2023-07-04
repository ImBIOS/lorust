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
/// let s = "Lorem ipsum dolor sit amet, consectetur adipiscing elit.";
/// let words = words(s, None);
/// assert_eq!(words, vec!["Lorem", "ipsum", "dolor", "sit", "amet", "consectetur", "adipiscing", "elit"]);
/// ```
pub fn words(s: &str, re: Option<&str>) -> Vec<String> {
    use regex::Regex;

    if s.is_empty() {
        return Vec::new();
    }

    let re = re.map(|rgx| Regex::new(rgx).unwrap());

    match re {
        Some(rgx) => rgx.find_iter(s).map(|w| w.as_str().to_string()).collect(),
        None => {
            let mut result = Vec::new();
            let mut word = String::new();

            for (_, c) in s.chars().enumerate() {
                if c.is_alphanumeric() {
                    word.push(c);
                } else if !word.is_empty() {
                    result.push(word.clone());
                    word.clear();
                }
            }

            if !word.is_empty() {
                result.push(word);
            }

            result
        }
    }
}
