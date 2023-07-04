#[cfg(test)]
mod test_string {
    use lorust::{capitalize, deburr, ends_with, kebab_case, words};

    #[test]
    fn test_capitalize() {
        // Test empty string
        assert_eq!(capitalize(""), "");

        // Test string with only one character
        assert_eq!(capitalize("a"), "A");
        assert_eq!(capitalize("z"), "Z");

        // Test string with all lowercase characters
        assert_eq!(capitalize("hello"), "Hello");

        // Test string with all uppercase characters
        assert_eq!(capitalize("WORLD"), "World");

        // Test string with mixed case characters
        assert_eq!(capitalize("GooD MORNING"), "Good morning");

        // Test string with non-alphabetic characters
        assert_eq!(capitalize("123abc!@#"), "123abc!@#");
    }

    #[test]
    fn test_deburr() {
        assert_eq!(deburr("déjà vu"), "deja vu");
        assert_eq!(deburr("Çédille"), "Cedille");
        assert_eq!(deburr("Ångström"), "Angstrom");
        assert_eq!(deburr("Äpfel"), "Apfel");
        assert_eq!(deburr("Être"), "Etre");
        assert_eq!(deburr("Österreich"), "Osterreich");
        assert_eq!(deburr("Pâté"), "Pate");
        assert_eq!(deburr("pëllë"), "pelle");
        assert_eq!(deburr("lòrèm ípsùm"), "lorem ipsum");
    }

    #[test]
    fn test_ends_with() {
        assert_eq!(ends_with("abc", "c", None), true);
        assert_eq!(ends_with("abc", "b", None), false);
        assert_eq!(ends_with("abc", "b", Some(2)), true);
        assert_eq!(ends_with("abc", "d", None), false);
        assert_eq!(ends_with("abc", "abc", Some(3)), true);
    }

    #[test]
    fn test_kebab_case() {
        assert_eq!(kebab_case("Foo Bar"), "foo-bar");
        assert_eq!(kebab_case("fooBar"), "foo-bar");
        assert_eq!(kebab_case("__FOO_BAR__"), "foo-bar");
        assert_eq!(kebab_case("foo bar_baz"), "foo-bar-baz");
        assert_eq!(kebab_case("FOO"), "foo");
        assert_eq!(kebab_case("foo"), "foo");
        assert_eq!(kebab_case("Foo1Bar"), "foo1-bar");
        assert_eq!(kebab_case("foo__Bar"), "foo-bar");
        assert_eq!(kebab_case("foo__BAr_baz"), "foo-bar-baz");
        assert_eq!(kebab_case(""), "");
        assert_eq!(kebab_case("HelloWorld"), "hello-world");
        assert_eq!(kebab_case("Hello   World"), "hello-world");
        assert_eq!(kebab_case("Hello___World"), "hello-world");
        assert_eq!(kebab_case("Hello---World"), "hello-world");
    }

    #[test]
    fn test_words_default_pattern() {
        assert_eq!(
            words("fred, barney, & pebbles", None),
            vec!["fred", "barney", "pebbles"]
        );
    }

    #[test]
    fn test_words_custom_pattern() {
        assert_eq!(
            words("fred, barney, & pebbles", Some(r"[^, ]+")),
            vec!["fred", "barney", "&", "pebbles"]
        );
    }
}
