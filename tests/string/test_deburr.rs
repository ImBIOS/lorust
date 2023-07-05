#[cfg(test)]
mod tests {
    use lorust::deburr;

    #[test]
    fn test_deburr_functional() {
        assert_eq!(deburr("déjà vu".to_string()), "deja vu");
        assert_eq!(deburr("Çédille".to_string()), "Cedille");
        assert_eq!(deburr("Ångström".to_string()), "Angstrom");
        assert_eq!(deburr("Äpfel".to_string()), "Apfel");
        assert_eq!(deburr("Être".to_string()), "Etre");
        assert_eq!(deburr("Österreich".to_string()), "Osterreich");
        assert_eq!(deburr("Pâté".to_string()), "Pate");
        assert_eq!(deburr("pëllë".to_string()), "pelle");
        assert_eq!(deburr("lòrèm ípsùm".to_string()), "lorem ipsum");
    }
}
