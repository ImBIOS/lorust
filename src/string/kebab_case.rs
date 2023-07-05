/// Converts a `String` to kebab case.
///
/// Kebab case is a string converted to lowercase with any spaces, underscores,
/// or camel case boundaries replaced by hyphens.
///
/// # Arguments
///
/// * `s` - The input string to be converted to kebab case.
///
/// # Returns
///
/// Returns a new string converted to kebab case.
///
/// # Examples
///
/// ```rust
/// use lorust::kebab_case;
///
/// let kebab_cased = kebab_case("Foo Bar".to_string());
/// assert_eq!(kebab_cased, "foo-bar");
/// let kebab_cased = kebab_case("fooBar".to_string());
/// assert_eq!(kebab_cased, "foo-bar");
/// let kebab_cased = kebab_case("__FOO_BAR__".to_string());
/// assert_eq!(kebab_cased, "foo-bar");
/// ```
pub fn kebab_case(s: String) -> String {
    use regex::Regex;

    // Trim leading/trailing whitespace, underscores, and hyphens
    let s = s.trim_matches(|c: char| c.is_whitespace() || c == '_' || c == '-');

    // Replace underscore or space with hyphen
    let re = Regex::new(r"[\s_]+").unwrap();
    let s = re.replace_all(s, "-");

    // Convert camel case boundaries to hyphen
    let re = Regex::new(r"([a-z0-9])([A-Z])").unwrap();
    let s = re.replace_all(&s, "$1-$2");

    // Convert double hyphens to single
    let re = Regex::new(r"-+").unwrap();
    let s = re.replace_all(&s, "-");

    // Remove non-word and non-hyphen characters
    let re = Regex::new(r"[^\w-]").unwrap();
    let s = re.replace_all(&s, "");

    s.to_lowercase()
}
