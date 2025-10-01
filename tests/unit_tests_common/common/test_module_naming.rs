#[cfg(test)]
pub mod module_naming_validation_tests {
    use std::fs;
    use std::path::Path;

    #[test]
    pub fn test_no_generic_mod_tests_in_unit() {
        let UNIT_TESTS_DIR = Path::new("tests/unit/common");

        if !UNIT_TESTS_DIR.exists() {
            return;
        }

        let mut violations = Vec::new();

        if let Ok(entries) = fs::read_dir(UNIT_TESTS_DIR) {
            for entry in entries.flatten() {
                let PATH = entry.path();
                if PATH.extension().is_some_and(|ext| ext == "rs")
                    && PATH.file_name().is_some_and(|name| name != "mod.rs")
                    && let Ok(content) = fs::read_to_string(&PATH)
                {
                    let PATTERN = format!("mod {} {{", "tests");
                    if content.contains(&PATTERN) {
                        violations.push(format!(
                            "{}: Uses generic 'mod tests' instead of unique module name",
                            PATH.display()
                        ));
                    }
                }
            }
        }

        assert!(
            violations.is_empty(),
            "Found generic 'mod tests' declarations in unit:\n{}",
            violations.join("\n")
        );
    }
}
