#[allow(clippy::all)]
#[allow(clippy::pedantic)]
#[allow(clippy::nursery)]
#[cfg(test)]
mod common_tests_display_tests {
    use std::fs;

    #[test]
    fn test_common_tests_display_validation() {
        let SCRIPT_CONTENT =
            fs::read_to_string("./scripts/test.sh").expect("test.sh file not found");

        // Check that script displays category names
        assert!(
            SCRIPT_CONTENT.contains("echo \"Category_name: $CATEGORY_NAME\""),
            "test.sh missing category name display"
        );

        // Check that script displays test type
        assert!(
            SCRIPT_CONTENT.contains("echo \"Test type: $test_type\""),
            "test.sh missing test type display"
        );

        // Check that script runs cargo test for each category
        assert!(
            SCRIPT_CONTENT.contains("cargo test --test \"$CATEGORY_NAME\" \"$TEST_NAME\""),
            "test.sh not running cargo test for categories"
        );

        // Check that script tracks test results
        assert!(
            SCRIPT_CONTENT.contains("TOTAL_PASSED=$((TOTAL_PASSED + 1))"),
            "test.sh not tracking passed tests"
        );

        // Check that script tracks failed tests
        assert!(
            SCRIPT_CONTENT.contains("TOTAL_FAILED=$((TOTAL_FAILED + 1))"),
            "test.sh not tracking failed tests"
        );

        // Check that script displays test summary
        assert!(
            SCRIPT_CONTENT.contains("echo \"Test Summary:\""),
            "test.sh missing test summary display"
        );
    }
}
