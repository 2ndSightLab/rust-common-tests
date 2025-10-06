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
            SCRIPT_CONTENT.contains("echo \"Test Summary:\""),
            "test.sh missing Test Summary section"
        );
        assert!(
            SCRIPT_CONTENT.contains("echo \"================================\""),
            "test.sh missing summary separator"
        );

        // Check that script displays total tests
        assert!(
            SCRIPT_CONTENT.contains("echo \"Total Tests: $TOTAL_TESTS\""),
            "test.sh not displaying total tests count"
        );

        // Check that script displays passed/failed counts
        assert!(
            SCRIPT_CONTENT.contains("echo \"Passed: $TOTAL_PASSED\""),
            "test.sh not displaying passed count"
        );

        assert!(
            SCRIPT_CONTENT.contains("echo \"Failed: $TOTAL_FAILED\""),
            "test.sh not displaying failed count"
        );

        // Check that script displays category counts
        assert!(
            SCRIPT_CONTENT.contains("echo \"Total Integration: $TOTAL_INTEGRATION\""),
            "test.sh not displaying integration count"
        );

        assert!(
            SCRIPT_CONTENT.contains("echo \"Total Security: $TOTAL_SECURITY\""),
            "test.sh not displaying security count"
        );

        assert!(
            SCRIPT_CONTENT.contains("echo \"Total Unit Tests: $TOTAL_UNIT\""),
            "test.sh not displaying unit tests count"
        );
    }
}
