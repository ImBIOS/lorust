#[cfg(test)]
mod tests {
    use lorust::capitalize;

    #[test]
    fn test_single_word() {
        assert_eq!(capitalize("FRED".to_string()), "Fred");
        assert_eq!(capitalize("fred".to_string()), "Fred");
        assert_eq!(capitalize("FrEd".to_string()), "Fred");
        assert_eq!(capitalize("f".to_string()), "F");
        assert_eq!(capitalize("F".to_string()), "F");
        assert_eq!(capitalize("fr".to_string()), "Fr");
        assert_eq!(capitalize("Fr".to_string()), "Fr");
        assert_eq!(capitalize("".to_string()), "");
        assert_eq!(capitalize(".".to_string()), ".");
        assert_eq!(capitalize("f.".to_string()), "F.");
        assert_eq!(capitalize("F.".to_string()), "F.");
    }

    #[test]
    fn test_multi_word() {
        assert_eq!(capitalize("HELLO WORLD".to_string()), "Hello world");
        assert_eq!(capitalize("hello world".to_string()), "Hello world");
        assert_eq!(capitalize("Hello World".to_string()), "Hello world");
        assert_eq!(capitalize("h W".to_string()), "H w");
        assert_eq!(capitalize("H W".to_string()), "H w");
    }

    #[test]
    fn test_special_chars() {
        assert_eq!(capitalize("!@#$%^&*()".to_string()), "!@#$%^&*()");
        assert_eq!(capitalize("1234567890".to_string()), "1234567890");
        assert_eq!(capitalize("a1b2c3d4".to_string()), "A1b2c3d4");
        assert_eq!(capitalize("A1B2C3D4".to_string()), "A1b2c3d4");
    }
}
