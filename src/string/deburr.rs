use unicode_normalization::UnicodeNormalization;

/// Deburrs a string by converting
/// [Latin-1 Supplement](https://en.wikipedia.org/wiki/Latin-1_Supplement_(Unicode_block)#Character_table)
/// and [Latin Extended-A](https://en.wikipedia.org/wiki/Latin_Extended-A)
/// letters to basic Latin letters and removing
/// [combining diacritical marks](https://en.wikipedia.org/wiki/Combining_Diacritical_Marks).
///
/// # Arguments
///
/// * `input` - A string slice that holds the input string.
///
/// # Returns
///
/// * A string without Latin-1 Supplement, Latin Extended-A letters and diacritical marks.
///
/// # Example
///
/// ```rust
/// use lorust::deburr;
///
/// let deburred = deburr("déjà vu".to_string());
/// assert_eq!(deburred, "deja vu");
/// ```
pub fn deburr(input: String) -> String {
    input
        .nfkd()
        .filter(|c| c.is_ascii() && !c.is_control())
        .collect::<String>()
}
