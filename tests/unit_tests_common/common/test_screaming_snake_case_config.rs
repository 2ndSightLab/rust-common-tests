#[cfg(test)]
mod tests {
    use regex::Regex;
    use std::fs;

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
}
