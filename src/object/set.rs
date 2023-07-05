/// Sets the value at `key` of `object`. If the `key` doesn't exist,
/// it's created.
///
/// **Note:** This function mutates `object`.
///
/// # Examples
///
/// ```
/// use lorust::set;
/// use std::collections::HashMap;
///
/// let mut object: HashMap<String, String> = HashMap::new();
/// set(&mut object, "key", "value");
/// assert_eq!(object.get("key").unwrap(), "value");
/// ```
pub fn set(object: &mut std::collections::HashMap<String, String>, key: &str, value: &str) {
    object.insert(key.to_string(), value.to_string());
}
