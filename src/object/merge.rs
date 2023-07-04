use serde_json::Value;

/// Merges two objects recursively, assigning properties from source objects to the destination object.
///
/// Properties that resolve to `None` in source objects are skipped if a corresponding property exists in the destination object.
/// Array and plain object properties are merged recursively.
/// Other objects and value types are overridden by assignment.
/// Source objects are applied from left to right. Subsequent sources overwrite property assignments of previous sources.
///
/// # Arguments
///
/// * `destination` - The destination object.
/// * `sources` - The source objects.
///
/// # Returns
///
/// The merged object.
///
/// # Example
///
/// ```rust
/// use lorust::merge;
///
/// let mut object = serde_json::json!({
///    "a": [
///       { "b": 2 },
///      { "d": 4 }
///   ]
/// });
///
/// let other = serde_json::json!({
///   "a": [
///     { "c": 3 },
///    { "e": 5 }
///  ]
/// });
///
/// let result = merge(&mut object, &[&other]);
/// let expected = serde_json::json!({
///  "a": [
///    { "b": 2, "c": 3 },
///   { "d": 4, "e": 5 }
/// ]
/// });
///
/// assert_eq!(result, expected);
/// ```
pub fn merge(destination: &mut Value, sources: &[&Value]) -> Value {
    for source in sources {
        if let (Some(destination_obj), Some(source_obj)) =
            (destination.as_object_mut(), source.as_object())
        {
            for (key, value) in source_obj {
                if let Some(destination_value) = destination_obj.get_mut(key) {
                    merge(destination_value, &[value]);
                } else {
                    destination_obj.insert(key.clone(), value.clone());
                }
            }
        } else if let (Some(destination_arr), Some(source_arr)) =
            (destination.as_array_mut(), source.as_array())
        {
            for (index, source_value) in source_arr.iter().enumerate() {
                if index < destination_arr.len() {
                    if let Some(destination_value) = destination_arr.get_mut(index) {
                        merge(destination_value, &[source_value]);
                    }
                } else {
                    destination_arr.push(source_value.clone());
                }
            }
        }
    }

    destination.clone()
}
