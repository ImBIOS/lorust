use std::collections::HashMap;

/// Creates a new HashMap with the same keys as `map` and values generated
/// by running each key-value pair of `map` through the `mapper` function.
///
/// # Arguments
///
/// * `map` - The HashMap to iterate over.
/// * `mapper` - The function invoked per iteration, taking a key-value pair and returning the mapped value.
///
/// # Returns
///
/// Returns a new HashMap with the mapped values.
///
/// # Examples
///
/// ```
/// use lorust::map_values;
/// use std::collections::HashMap;
///
/// let mut users = HashMap::new();
/// users.insert("fred", 40);
/// users.insert("pebbles", 1);
///
/// let result = map_values(&users, |(_, &value)| value * 2);
///
/// assert_eq!(result.get("fred"), Some(&80));
/// assert_eq!(result.get("pebbles"), Some(&2));
/// ```
pub fn map_values<K, V, F>(map: &HashMap<K, V>, mapper: F) -> HashMap<K, V>
where
    K: std::cmp::Eq + std::hash::Hash + std::clone::Clone,
    V: std::clone::Clone,
    F: Fn((&K, &V)) -> V,
{
    let mut result = HashMap::new();

    for (key, value) in map {
        let mapped_value = mapper((key, value));
        result.insert(key.clone(), mapped_value);
    }

    result
}
