#[cfg(test)]
mod test_file_permissions_tests {
    use std::fs;
    use walkdir::WalkDir;

    #[test]
    fn test_no_unsafe_file_permissions() {
        let CURRENT_DIR = std::env::current_dir().expect("Failed to get current directory");

        // Skip if we're in the common tests directory itself
        if CURRENT_DIR.file_name().unwrap_or_default() == "rust-common-tests" {
            return;
        }

        for ENTRY in WalkDir::new(&CURRENT_DIR)
            .into_iter()
            .filter_map(std::result::Result::ok)
            .filter(|E| E.path().extension().is_some_and(|EXT| EXT == "rs"))
        {
            let Ok(CONTENT) = fs::read_to_string(ENTRY.path()) else {
                continue;
            };

            // Check for world-writable permissions
            let UNSAFE_PATTERNS = ["0o777", "0o666", "0o766", "0o676", "0o667"];

            for PATTERN in &UNSAFE_PATTERNS {
                assert!(
                    !CONTENT.contains(PATTERN),
                    "Unsafe file permission {PATTERN} found in {:?}",
                    ENTRY.path()
                );
            }
        }
    }
}
