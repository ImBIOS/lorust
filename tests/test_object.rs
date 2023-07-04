#[cfg(test)]
mod test_object {
    use lorust::{map_values, merge, pick};
    use std::collections::HashMap;

    #[test]
    fn test_map_values() {
        let mut users = HashMap::new();
        users.insert("fred", 40);
        users.insert("pebbles", 1);

        let result = map_values(&users, |(_, &value)| value * 2);

        assert_eq!(result.get("fred"), Some(&80));
        assert_eq!(result.get("pebbles"), Some(&2));
    }

    #[test]
    fn test_merge() {
        let mut object = serde_json::json!({
            "a": [
                { "b": 2 },
                { "d": 4 }
            ]
        });

        let other = serde_json::json!({
            "a": [
                { "c": 3 },
                { "e": 5 }
            ]
        });

        let result = merge(&mut object, &[&other]);
        let expected = serde_json::json!({
            "a": [
                { "b": 2, "c": 3 },
                { "d": 4, "e": 5 }
            ]
        });

        assert_eq!(result, expected);
    }

    #[test]
    fn test_pick() {
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
