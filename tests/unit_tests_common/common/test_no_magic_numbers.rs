#[cfg(test)]
mod tests {
    use regex::Regex;
    use std::fs;
    use walkdir::WalkDir;

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
            .filter(|E| !E.path().to_string_lossy().contains("/tests/"))
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
