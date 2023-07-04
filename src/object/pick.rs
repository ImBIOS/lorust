use std::collections::HashMap;

/// Creates a new HashMap composed of the picked properties from the source HashMap.
///
/// # Arguments
///
/// * `object` - The source HashMap.
/// * `paths` - The property keys to pick.
///
/// # Returns
///
/// A new HashMap containing the picked properties.
///
/// # Examples
///
/// ```
/// use lorust::pick;
/// use std::collections::HashMap;
///
/// let mut object = HashMap::new();
/// let binding = "1".to_string();
/// object.insert("a", &binding);
/// let binding = "2".to_string();
/// object.insert("b", &binding);
/// let binding = "3".to_string();
/// object.insert("c", &binding);
///
/// let picked = pick(&object, &["a", "c"]);
///
/// assert_eq!(picked.get("a"), Some(&String::from("1")));
/// assert_eq!(picked.get("c"), Some(&String::from("3")));
/// assert_eq!(picked.get("b"), None);
/// ```
pub fn pick(object: &HashMap<&str, &String>, paths: &[&str]) -> HashMap<String, String> {
    let mut picked = HashMap::new();

    for path in paths {
        if let Some(value) = object.get(path) {
            picked.insert(path.to_string(), value.to_string());
        }
    }

    picked
}
