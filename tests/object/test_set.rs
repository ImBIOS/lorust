#[cfg(test)]
mod tests {
    use lorust::set;
    use std::collections::HashMap;

    #[test]
    fn test_set_functional() {
        let mut object: HashMap<String, String> = HashMap::new();

        // Test setting a new key-value pair
        set(&mut object, "key", "value");
        assert_eq!(object.get("key"), Some(&"value".to_string()));

        // Test overwriting an existing key-value pair
        set(&mut object, "key", "new_value");
        assert_eq!(object.get("key"), Some(&"new_value".to_string()));

        // Test setting multiple key-value pairs
        set(&mut object, "key2", "value2");
        set(&mut object, "key3", "value3");
        assert_eq!(object.get("key2"), Some(&"value2".to_string()));
        assert_eq!(object.get("key3"), Some(&"value3".to_string()));
    }
}
