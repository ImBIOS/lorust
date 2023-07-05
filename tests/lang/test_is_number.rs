#[cfg(test)]
mod tests {
    use lorust::is_number;

    #[test]
    fn test_valid_numbers() {
        assert!(is_number("123".to_string()));
        assert!(is_number("123.456".to_string()));
        assert!(is_number("-123.456".to_string()));
        assert!(is_number("+123.456".to_string()));
        assert!(is_number("0".to_string()));
        assert!(is_number("-0".to_string()));
        assert!(is_number("+0".to_string()));
    }

    #[test]
    fn test_invalid_strings() {
        assert!(!is_number("abc".to_string()));
        assert!(!is_number("123abc".to_string()));
        assert!(!is_number("123.456.789".to_string()));
        assert!(!is_number("".to_string()));
        assert!(!is_number(" ".to_string()));
        assert!(!is_number("+".to_string()));
        assert!(!is_number("-".to_string()));
        assert!(!is_number(".".to_string()));
    }

    #[test]
    fn test_special_cases() {
        assert!(!is_number("inf".to_string()));
        assert!(!is_number("-inf".to_string()));
        assert!(!is_number("+inf".to_string()));
        assert!(!is_number("infinity".to_string()));
        assert!(!is_number("-infinity".to_string()));
        assert!(!is_number("+infinity".to_string()));
        assert!(!is_number("nan".to_string()));
        assert!(!is_number("NaN".to_string()));
        assert!(!is_number("NAN".to_string()));
    }

    #[test]
    fn test_extreme_numbers() {
        assert!(is_number("1e308".to_string()));
        assert!(is_number("1e309".to_string()));
        assert!(is_number("-1e308".to_string()));
        assert!(is_number("-1e309".to_string()));
    }
}
