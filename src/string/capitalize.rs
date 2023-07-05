/// Converts the first character of the input string to uppercase and the remaining characters to lowercase.
///
/// # Arguments
///
/// * `s` - The input string to be capitalized.
///
/// # Returns
///
/// Returns a new string with the first character capitalized and the remaining characters in lowercase.
///
/// # Example
///
/// ```rust
/// use lorust::capitalize;
///
/// let capitalized = capitalize("FRED".to_string());
/// assert_eq!(capitalized, "Fred");
/// ```
pub fn capitalize(s: String) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => {
            first.to_uppercase().collect::<String>() + chars.as_str().to_lowercase().as_str()
        }
    }
}
