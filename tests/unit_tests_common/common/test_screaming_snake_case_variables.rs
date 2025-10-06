#[cfg(test)]
mod tests {
    use regex::Regex;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_screaming_snake_case_variables() {
        fn check_file(PATH: &Path, REGEX: &Regex, VIOLATIONS: &mut Vec<String>) {
            if let Ok(CONTENT) = fs::read_to_string(PATH) {
                for (LINE_NUM, LINE) in CONTENT.lines().enumerate() {
                    if let Some(CAPTURES) = REGEX.captures(LINE) {
                        let VAR_NAME = &CAPTURES[1];
                        // Skip if it's already SCREAMING_SNAKE_CASE
                        if !VAR_NAME
                            .chars()
                            .all(|c| c.is_uppercase() || c.is_numeric() || c == '_')
                        {
                            VIOLATIONS.push(format!(
                                "{}:{}: Variable '{}' not in SCREAMING_SNAKE_CASE",
                                PATH.display(),
                                LINE_NUM + 1,
                                VAR_NAME
                            ));
                        }
                    }
                }
            }
        }

        fn walk_dir(DIR: &Path, REGEX: &Regex, VIOLATIONS: &mut Vec<String>) {
            if let Ok(ENTRIES) = fs::read_dir(DIR) {
                for ENTRY in ENTRIES.flatten() {
                    let PATH = ENTRY.path();
                    if PATH.is_dir() {
                        walk_dir(&PATH, REGEX, VIOLATIONS);
                    } else if PATH.extension().is_some_and(|ext| ext == "rs") {
                        check_file(&PATH, REGEX, VIOLATIONS);
                    }
                }
            }
        }

        let SRC_DIR = Path::new("src");
        let TESTS_DIR = Path::new("tests");

        assert!(SRC_DIR.exists(), "Source directory not found");

        // Match variable declarations that are NOT in SCREAMING_SNAKE_CASE
        let NON_SCREAMING_REGEX =
            Regex::new(r"let\s+([a-z][a-zA-Z0-9_]*|[A-Z][a-z][a-zA-Z0-9_]*)\s*[=:]").unwrap();
        let mut VIOLATIONS = Vec::new();

        walk_dir(SRC_DIR, &NON_SCREAMING_REGEX, &mut VIOLATIONS);

        if TESTS_DIR.exists() {
            walk_dir(TESTS_DIR, &NON_SCREAMING_REGEX, &mut VIOLATIONS);
        }

        assert!(
            VIOLATIONS.is_empty(),
            "Found variables not in SCREAMING_SNAKE_CASE:\n{}",
            VIOLATIONS.join("\n")
        );
    }
}
