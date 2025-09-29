#[cfg(test)]
mod config_standards_tests {
    use regex::Regex;
    use std::fs;
    use walkdir::WalkDir;

    #[test]
    fn test_screaming_snake_case_config() {
        let CURRENT_DIR = std::env::current_dir().expect("Failed to get current directory");

        // Skip if we're in the common tests directory itself
        if CURRENT_DIR.file_name().unwrap_or_default() == "rust-common-tests" {
            return;
        }

        let CONFIG_FILES = ["service.toml", "config/action.toml"];
        let KEY_REGEX = Regex::new(r"^([A-Z][A-Z0-9_]*)\s*=").unwrap();

        for CONFIG_FILE in &CONFIG_FILES {
            let CONFIG_PATH = CURRENT_DIR.join(CONFIG_FILE);
            if CONFIG_PATH.exists() {
                let CONTENT = fs::read_to_string(&CONFIG_PATH).unwrap();

                // Check that all config keys are SCREAMING_SNAKE_CASE
                for LINE in CONTENT.lines() {
                    let TRIMMED = LINE.trim();
                    if TRIMMED.starts_with('#') || TRIMMED.is_empty() {
                        continue;
                    }

                    if let Some(CAPTURES) = KEY_REGEX.captures(TRIMMED) {
                        let KEY = &CAPTURES[1];
                        assert!(
                            KEY.chars()
                                .all(|C| C.is_ascii_uppercase() || C.is_ascii_digit() || C == '_'),
                            "Config key '{KEY}' in {CONFIG_FILE} is not SCREAMING_SNAKE_CASE"
                        );
                    }
                }
            }
        }
    }

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

    #[test]
    fn test_no_magic_numbers() {
        let CURRENT_DIR = std::env::current_dir().expect("Failed to get current directory");

        // Skip if we're in the common tests directory itself
        if CURRENT_DIR.file_name().unwrap_or_default() == "rust-common-tests" {
            return;
        }

        // Common magic numbers to avoid (excluding 0, 1, and common array indices)
        let MAGIC_NUMBER_REGEX =
            Regex::new(r"\b(?:80|443|8080|3000|5000|8000|9000|22|21|25|53|110|143|993|995)\b")
                .unwrap();

        for ENTRY in WalkDir::new(&CURRENT_DIR)
            .into_iter()
            .filter_map(std::result::Result::ok)
            .filter(|E| E.path().extension().is_some_and(|EXT| EXT == "rs"))
        {
            let Ok(CONTENT) = fs::read_to_string(ENTRY.path()) else {
                continue;
            };

            // Skip comments and string literals for this check
            for LINE in CONTENT.lines() {
                let TRIMMED = LINE.trim();
                if TRIMMED.starts_with("//") || TRIMMED.contains('"') {
                    continue;
                }

                assert!(
                    !MAGIC_NUMBER_REGEX.is_match(TRIMMED),
                    "Potential magic number found in {:?}: {TRIMMED}",
                    ENTRY.path()
                );
            }
        }
    }
}
