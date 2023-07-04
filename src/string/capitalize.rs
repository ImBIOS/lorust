/// Converts the first character of the input string to uppercase and the remaining characters to lowercase.
///
/// # Arguments
///
/// * `s` - The input string to be capitalized.
///
/// # Returns
///
/// Returns a new string with the first character capitalized and the remaining characters in lowercase.
/// If the input string is empty or None, returns None.
///
/// # Example
///
/// ```rust
/// use lorust::capitalize;
///
/// let capitalized = capitalize("FRED");
/// assert_eq!(capitalized, "Fred");
/// ```
pub fn capitalize(string: &str) -> String {
    if let Some(first) = string.chars().next() {
        let mut capitalized = first.to_uppercase().to_string();
        capitalized.push_str(&string[1..].to_lowercase());
        capitalized
    } else {
        String::new()
    }
}
