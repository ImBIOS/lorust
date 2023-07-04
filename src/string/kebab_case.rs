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
/// let kebab_cased = kebab_case("Foo Bar");
/// assert_eq!(kebab_cased, "foo-bar");
/// let kebab_cased = kebab_case("fooBar");
/// assert_eq!(kebab_cased, "foo-bar");
/// let kebab_cased = kebab_case("__FOO_BAR__");
/// assert_eq!(kebab_cased, "foo-bar");
/// ```
pub fn kebab_case(s: &str) -> String {
    use regex::Regex;
    let s = s.trim_matches(|c: char| c.is_whitespace() || c == '_');
    let re = Regex::new(r"([a-z0-9])([A-Z])").unwrap();
    let s1 = re.replace_all(s, "$1-$2");
    let re = Regex::new(r"[\s_]+").unwrap();
    let s2 = re.replace_all(&s1, "-");
    let re = Regex::new(r"-+").unwrap();
    let s3 = re.replace_all(&s2, "-");
    s3.to_lowercase()
}
