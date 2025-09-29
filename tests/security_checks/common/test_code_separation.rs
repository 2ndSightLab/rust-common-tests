#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_no_test_code_in_src() {
        fn check_file_for_test_code(PATH: &Path) -> Vec<String> {
            let mut VIOLATIONS = Vec::new();
            if let Ok(CONTENT) = fs::read_to_string(PATH) {
                if CONTENT.contains("#[cfg(test)]") {
                    VIOLATIONS.push(format!("{}: Contains #[cfg(test)]", PATH.display()));
                }
                if CONTENT.contains("#[test]") {
                    VIOLATIONS.push(format!("{}: Contains #[test]", PATH.display()));
                }
            }
            VIOLATIONS
        }

        fn walk_src_dir(DIR: &Path) -> Vec<String> {
            let mut VIOLATIONS = Vec::new();
            if let Ok(ENTRIES) = fs::read_dir(DIR) {
                for ENTRY in ENTRIES.flatten() {
                    let PATH = ENTRY.path();
                    if PATH.is_dir() {
                        VIOLATIONS.extend(walk_src_dir(&PATH));
                    } else if PATH.extension().is_some_and(|ext| ext == "rs") {
                        VIOLATIONS.extend(check_file_for_test_code(&PATH));
                    }
                }
            }
            VIOLATIONS
        }

        let SRC_DIR = Path::new("src");
        assert!(SRC_DIR.exists(), "src directory not found");

        let VIOLATIONS = walk_src_dir(SRC_DIR);
        assert!(
            VIOLATIONS.is_empty(),
            "Test code found in src: {VIOLATIONS:?}"
        );
    }
}
