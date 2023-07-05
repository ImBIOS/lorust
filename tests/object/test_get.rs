#[cfg(test)]
mod tests {
    use lorust::get;
    use std::collections::HashMap;

    #[test]
    fn test_get_functional() {
        let mut object = HashMap::new();
        object.insert(String::from("a"), 3);
        object.insert(String::from("b"), 5);

        assert_eq!(get(&object, "a", &0), Ok(&3));
        assert_eq!(get(&object, "b", &0), Ok(&5));
        assert_eq!(get(&object, "c", &0), Ok(&0));
    }

    #[test]
    fn test_get_with_empty_object() {
        let object = HashMap::new();

        assert_eq!(get(&object, "a", &0), Ok(&0));
    }

    #[test]
    fn test_get_with_empty_path() {
        let mut object = HashMap::new();
        object.insert(String::from("a"), 3);

        assert_eq!(get(&object, "", &0), Ok(&0));
    }

    #[test]
    fn test_get_with_empty_path_and_empty_object() {
        let object = HashMap::new();

        assert_eq!(get(&object, "", &0), Ok(&0));
    }
}
