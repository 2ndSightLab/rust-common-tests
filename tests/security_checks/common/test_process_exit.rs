#[cfg(test)]
mod test_process_exit_tests {
    use std::fs;
    use walkdir::WalkDir;

    #[test]
    fn test_no_process_exit_calls() {
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

            // Check for process::exit calls (security vulnerability)
            assert!(
                !CONTENT.contains("process::exit"),
                "process::exit() call found in {:?} - use proper error handling instead",
                ENTRY.path()
            );
            assert!(
                !CONTENT.contains("std::process::exit"),
                "std::process::exit() call found in {:?} - use proper error handling instead",
                ENTRY.path()
            );
        }
    }
}
