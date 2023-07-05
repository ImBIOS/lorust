#[cfg(test)]
mod tests {
    use lorust::map_values;
    use std::collections::HashMap;

    #[test]
    fn test_map_values_functional() {
        let mut users = HashMap::new();
        users.insert("fred", 40);
        users.insert("pebbles", 1);

        let result = map_values(&users, |(_, &value)| value * 2);

        assert_eq!(result.get("fred"), Some(&80));
        assert_eq!(result.get("pebbles"), Some(&2));
    }
}
