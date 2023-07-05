#[cfg(test)]
mod tests {
    use lorust::words;

    #[test]
    fn test_words_default_pattern() {
        assert_eq!(
            words("fred, barney, & pebbles".to_string(), None),
            vec!["fred", "barney", "pebbles"]
        );
    }

    #[test]
    fn test_words_custom_pattern() {
        assert_eq!(
            words("fred, barney, & pebbles".to_string(), Some(r"[^, ]+")),
            vec!["fred", "barney", "&", "pebbles"]
        );
    }

    #[test]
    fn test_words_empty_string() {
        assert_eq!(words("".to_string(), None), Vec::<String>::new());
    }
}
