#[cfg(test)]
mod tests {
    use lorust::pick;
    use std::collections::HashMap;

    #[test]
    fn test_pick_functional() {
        let mut object = HashMap::new();
        let binding = "1".to_string();
        object.insert("a", &binding);
        let binding = "2".to_string();
        object.insert("b", &binding);
        let binding = "3".to_string();
        object.insert("c", &binding);

        let picked = pick(&object, &["a", "c"]);

        assert_eq!(picked.get("a"), Some(&String::from("1")));
        assert_eq!(picked.get("c"), Some(&String::from("3")));
        assert_eq!(picked.get("b"), None);
    }
}
