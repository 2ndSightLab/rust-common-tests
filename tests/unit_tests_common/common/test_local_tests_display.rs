#[allow(clippy::all)]
#[allow(clippy::pedantic)]
#[allow(clippy::nursery)]
#[cfg(test)]
mod local_tests_display_tests {
    use std::fs;

    #[test]
    fn test_local_tests_display_validation() {
        let SCRIPT_CONTENT =
            fs::read_to_string("./scripts/test.sh").expect("test.sh file not found");

        // Check that script displays "Done testing" for each category
        assert!(
            SCRIPT_CONTENT.contains("echo \"Done testing: $CATEGORY_NAME\""),
            "test.sh missing done testing display"
        );

        // Check that script counts common vs local tests
        assert!(
            SCRIPT_CONTENT.contains("TOTAL_COMMON=$((TOTAL_COMMON + 1))"),
            "test.sh not counting common tests"
        );

        // Check that script counts local tests
        assert!(
            SCRIPT_CONTENT.contains("TOTAL_LOCAL=$((TOTAL_LOCAL + 1))"),
            "test.sh not counting local tests"
        );

        // Check that script displays total counts
        assert!(
            SCRIPT_CONTENT.contains("echo \"Total Common: $TOTAL_COMMON\""),
            "test.sh not displaying total common count"
        );

        // Check that script displays total local count
        assert!(
            SCRIPT_CONTENT.contains("echo \"Total Local: $TOTAL_LOCAL\""),
            "test.sh not displaying total local count"
        );
    }
}
