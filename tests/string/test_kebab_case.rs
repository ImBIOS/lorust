#[cfg(test)]
mod tests {
    use lorust::kebab_case;

    #[test]
    fn test_kebab_case() {
        assert_eq!(kebab_case("Foo Bar".to_string()), "foo-bar");
        assert_eq!(kebab_case("fooBar".to_string()), "foo-bar");
        assert_eq!(kebab_case("__FOO_BAR__".to_string()), "foo-bar");
        assert_eq!(kebab_case("foo bar".to_string()), "foo-bar");
        assert_eq!(kebab_case("foo -bar".to_string()), "foo-bar");
        assert_eq!(kebab_case("foo- bar".to_string()), "foo-bar");
        assert_eq!(kebab_case("foo - bar".to_string()), "foo-bar");
        assert_eq!(kebab_case("foo_bar".to_string()), "foo-bar");
        assert_eq!(kebab_case("foo__bar".to_string()), "foo-bar");
        assert_eq!(kebab_case("--foo--bar--".to_string()), "foo-bar");
        assert_eq!(kebab_case("FOO-BAR".to_string()), "foo-bar");
        assert_eq!(kebab_case("FOÈ-BAR".to_string()), "foè-bar");
        assert_eq!(kebab_case("-foo-bar-".to_string()), "foo-bar");
        assert_eq!(kebab_case("--foo--bar".to_string()), "foo-bar");
        assert_eq!(kebab_case("foo bar?".to_string()), "foo-bar");
        assert_eq!(kebab_case("foo bar!".to_string()), "foo-bar");
        assert_eq!(kebab_case("foo bar$".to_string()), "foo-bar");
        assert_eq!(kebab_case("foo-bar#".to_string()), "foo-bar");
    }
}
