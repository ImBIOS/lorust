/// The `is_number` function takes a string and returns a boolean value.
/// It checks if the input string can be parsed into a f64 number excluding the special f64 number cases
/// (i.e., "inf", "-inf", "+inf", "infinity", "-infinity", "+infinity", "nan").
///
/// # Arguments
///
/// * `s` - A String to check if it can be parsed into a f64 number excluding special cases.
///
/// # Example
///
/// ```
/// use lorust::is_number;
///
/// assert_eq!(is_number("123.456".to_string()), true);
/// assert_eq!(is_number("abc".to_string()), false);
/// assert_eq!(is_number("inf".to_string()), false);
/// assert_eq!(is_number("NaN".to_string()), false);
/// ```
pub fn is_number(s: String) -> bool {
    match s.to_lowercase().as_str() {
        "inf" | "-inf" | "+inf" | "infinity" | "-infinity" | "+infinity" | "nan" => return false,
        _ => {}
    }

    s.parse::<f64>().is_ok()
}
