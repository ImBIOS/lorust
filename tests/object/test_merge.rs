#[cfg(test)]
mod tests {
    use lorust::merge;

    #[test]
    fn test_merge_functional() {
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
}
