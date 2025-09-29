#[cfg(test)]
mod test_dependency_validation_tests {
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_all_dependencies_declared() {
        let CARGO_PATH = Path::new("Cargo.toml");
        assert!(CARGO_PATH.exists(), "Cargo.toml not found");

        let CARGO_CONTENT = fs::read_to_string(CARGO_PATH).unwrap();

        // Check that essential dependencies are declared
        assert!(CARGO_CONTENT.contains("libc ="), "Missing libc dependency");
        assert!(
            CARGO_CONTENT.contains("thiserror ="),
            "Missing thiserror dependency"
        );
        assert!(CARGO_CONTENT.contains("log ="), "Missing log dependency");
        assert!(
            CARGO_CONTENT.contains("serde ="),
            "Missing serde dependency"
        );
    }
}
