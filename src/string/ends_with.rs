/// Checks if `string` ends with the given `target` string.
///
/// # Arguments
///
/// * `string` - The string to inspect.
/// * `target` - The string to search for.
/// * `position` - An Option representing the position to search up to.
///     If no value is provided, the function will search up to the end of the `string`.
///
/// # Returns
///
/// Returns true if the `string` ends with the `target` string, otherwise false.
///
/// # Example
///
/// ```rust
/// use lorust::ends_with;
///
/// let ends_with = ends_with("abc", "c", None);
/// assert_eq!(ends_with, true);
/// ```
pub fn ends_with(string: &str, target: &str, position: Option<usize>) -> bool {
    // Determine the position to end the search
    let end_position = match position {
        Some(pos) => pos.min(string.len()),
        None => string.len(),
    };

    // If the target string is larger than the position, it can't be at the end
    if target.len() > end_position {
        return false;
    }

    // Determine the start position of the target string within the original string
    let start_position = end_position - target.len();

    // Get the substring from the original string
    let substring = &string[start_position..end_position];

    // Check if the substring equals the target string
    substring == target
}
