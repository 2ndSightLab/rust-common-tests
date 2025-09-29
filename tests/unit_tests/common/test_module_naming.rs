#[cfg(test)]
mod module_naming_validation_tests {
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_no_generic_mod_tests_in_unit_tests() {
        let unit_tests_dir = Path::new("tests/unit_tests/common");
        
        if !unit_tests_dir.exists() {
            return;
        }

        let mut violations = Vec::new();
        
        if let Ok(entries) = fs::read_dir(unit_tests_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "rs") && 
                   path.file_name().map_or(false, |name| name != "mod.rs") {
                    
                    if let Ok(content) = fs::read_to_string(&path) {
                        let pattern = format!("mod {} {{", "tests");
                        if content.contains(&pattern) {
                            violations.push(format!("{}: Uses generic 'mod tests' instead of unique module name", 
                                path.display()));
                        }
                    }
                }
            }
        }

        if !violations.is_empty() {
            panic!("Found generic 'mod tests' declarations in unit_tests:\n{}", 
                violations.join("\n"));
        }
    }
}
