#[cfg(test)]
mod test_object {
    use lorust::round;

    #[test]
    fn test_round_without_precision() {
        assert_eq!(round(4.006, 0), 4.0);
        assert_eq!(round(4.2, 0), 4.0);
        assert_eq!(round(4.8, 0), 5.0);
    }

    #[test]
    fn test_round_with_positive_precision() {
        assert_eq!(round(4.006, 2), 4.01);
        assert_eq!(round(4.2, 1), 4.2);
        assert_eq!(round(4.8, 1), 4.8);
    }

    #[test]
    fn test_round_with_negative_precision() {
        assert_eq!(round(4060.0, -2), 4100.0);
        assert_eq!(round(42.0, -1), 40.0);
        assert_eq!(round(1234.5678, -3), 1000.0);
    }
}
