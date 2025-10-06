#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn test_cargo_allows_screaming_snake_case() {
        let CURRENT_DIR = std::env::current_dir().expect("Failed to get current directory");

        // Skip if we're in the common tests directory itself
        if CURRENT_DIR.file_name().unwrap_or_default() == "rust-common-tests" {
            return;
        }

        let CARGO_PATH = CURRENT_DIR.join("Cargo.toml");
        if CARGO_PATH.exists() {
            let CONTENT = fs::read_to_string(&CARGO_PATH).unwrap();

            // Check for the required lint override
            assert!(
                CONTENT.contains("[lints.rust]") && CONTENT.contains("non_snake_case = \"allow\""),
                "Cargo.toml must contain [lints.rust] section with non_snake_case = \"allow\""
            );
        }
    }
}
