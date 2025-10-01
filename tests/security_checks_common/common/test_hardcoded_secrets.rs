#[cfg(test)]
mod test_hardcoded_secrets_tests {
    use regex::Regex;
    use std::fs;
    use walkdir::WalkDir;

    #[test]
    fn test_no_hardcoded_secrets() {
        let CURRENT_DIR = std::env::current_dir().expect("Failed to get current directory");

        // Skip if we're in the common tests directory itself
        if CURRENT_DIR.file_name().unwrap_or_default() == "rust-common-tests" {
            return;
        }

        let SECRET_PATTERNS = [
            r#"password\s*=\s*""#,
            r#"secret\s*=\s*""#,
            r#"api_key\s*=\s*""#,
            r#"token\s*=\s*""#,
            r#"key\s*=\s*"[^<]"#,
        ];

        for ENTRY in WalkDir::new(&CURRENT_DIR)
            .into_iter()
            .filter_map(std::result::Result::ok)
            .filter(|E| E.path().extension().is_some_and(|EXT| EXT == "rs"))
        {
            let Ok(CONTENT) = fs::read_to_string(ENTRY.path()) else {
                continue;
            };

            for PATTERN in &SECRET_PATTERNS {
                let REGEX = Regex::new(PATTERN).unwrap();
                assert!(
                    !REGEX.is_match(&CONTENT),
                    "Potential hardcoded secret found in {:?}",
                    ENTRY.path()
                );
            }
        }
    }
}
