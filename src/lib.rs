pub fn validate_name(name: &str) -> Result<Option<&str>, &'static str> {
    #[allow(non_snake_case)]
    let MAX_NAME_LENGTH: usize = 30;

    match name {
        "" => Ok(None),
        n if n.len() > MAX_NAME_LENGTH => Err("Name is too long"),
        n if !n.chars().all(|c| c.is_alphabetic() || c.is_whitespace()) => {
            Err("Name contains invalid characters")
        }
        n => Ok(Some(n)),
    }
}

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
        assert_eq!(result, Err("Name is too long"));
    }

    #[test]
    fn invalid_characters_in_name_returns_error() {
        let result = validate_name("Alice@123");
        assert_eq!(result, Err("Name contains invalid characters"));
    }
}
