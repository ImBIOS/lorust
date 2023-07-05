use std::collections::HashMap;

/// Gets the value at `path` of `object`. If the resolved value is
/// `None`, the `default_value` is returned in its place.
///
/// # Arguments
///
/// * `object` - The HashMap object to query.
/// * `path` - The string path of the property to get.
/// * `default_value` - The value returned for `None` resolved values.
///
/// # Examples
///
/// ```rust
/// use std::collections::HashMap;
/// use lorust::get;
///
/// let mut object = HashMap::new();
/// object.insert(String::from("a"), 3);
/// object.insert(String::from("b"), 5);
///
/// assert_eq!(get(&object, "a", &0), Ok(&3));
/// assert_eq!(get(&object, "b", &0), Ok(&5));
/// assert_eq!(get(&object, "c", &0), Ok(&0));
/// ```
pub fn get<'a>(
    object: &'a HashMap<String, i32>,
    path: &str,
    default_value: &'a i32,
) -> Result<&'a i32, &'static str> {
    match object.get(path) {
        Some(value) => Ok(value),
        None => Ok(default_value),
    }
}
