#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_name_returns_some() {
        let result = validate_name("Alice");
        assert_eq!(result, Ok(Some("Alice")))
    }

    #[test]
    fn empty_name_returns_none() {
        let result = validate_name("");
        assert_eq!(result, Ok(None))
    }

    #[test]
    fn too_long_name_returns_error() {
        let result = validate_name("A very long name that exceeds the limit");
        assert!(result, Err("Name is too long"));
    }

    #[test]
    fn invalid_characters_in_name_returns_error() {
        let result = validate_name("Alice@123");
        assert!(result, Err("Name contains invalid characters"));
    }
}
