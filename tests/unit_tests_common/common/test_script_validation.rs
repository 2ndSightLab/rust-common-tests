#[allow(clippy::all)]
#[allow(clippy::pedantic)]
#[allow(clippy::nursery)]
#[cfg(test)]
mod script_validation_tests {
    use std::fs;

    #[test]
    fn test_script_format_validation() {
        let SCRIPT_CONTENT =
            fs::read_to_string("./scripts/test.sh").expect("test.sh file not found");

        // Verify the script contains the required output format
        assert!(
            SCRIPT_CONTENT.contains("Test Results Summary:"),
            "test.sh missing Test Results Summary section"
        );
        assert!(
            SCRIPT_CONTENT.contains("===================="),
            "test.sh missing summary separator"
        );

        // Check that script generates category names from test files
        assert!(
            SCRIPT_CONTENT.contains(
                "TEST_NAME=$(echo \"${TEST_FILE}\" | sed 's/_/ /g' | sed 's/\\b\\w/\\U&/g')"
            ),
            "test.sh not generating category names from test files"
        );

        // Check for proper output format generation with PASSED status
        assert!(
            SCRIPT_CONTENT.contains("echo -e \"✅ ${TEST_NAME}: ${GREEN}PASSED${NC} ($TEST_PASSED passed, $TEST_FAILED failed)\""),
            "test.sh not generating correct PASSED format"
        );

        // Check for proper output format generation with FAILED status
        assert!(
            SCRIPT_CONTENT.contains("echo -e \"❌ ${TEST_NAME}: ${RED}FAILED${NC} ($TEST_PASSED passed, $TEST_FAILED failed)\""),
            "test.sh not generating correct FAILED format"
        );

        // Check for conditional logic based on test results
        assert!(
            SCRIPT_CONTENT.contains("if [[ $TEST_FAILED -eq 0 ]]; then"),
            "test.sh missing conditional logic for PASSED/FAILED status"
        );
    }
}
