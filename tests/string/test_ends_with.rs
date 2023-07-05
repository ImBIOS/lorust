#[cfg(test)]
mod tests {
    use lorust::ends_with;

    #[test]
    fn test_ends_with_functional() {
        assert!(ends_with("abc".to_string(), "c".to_string(), None));
        assert!(!ends_with("abc".to_string(), "b".to_string(), None));
        assert!(ends_with("abc".to_string(), "b".to_string(), Some(2)));
        assert!(!ends_with("abc".to_string(), "d".to_string(), None));
        assert!(ends_with("abc".to_string(), "abc".to_string(), Some(3)));
    }
}
