#[cfg(test)]
mod test_script_validation_tests {
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

        // Check for color definitions
        assert!(
            SCRIPT_CONTENT.contains("RED="),
            "test.sh missing RED color definition"
        );
        assert!(
            SCRIPT_CONTENT.contains("GREEN="),
            "test.sh missing GREEN color definition"
        );

        // Check for proper test result formatting with colors
        assert!(
            SCRIPT_CONTENT.contains("${GREEN}"),
            "test.sh missing GREEN color usage"
        );
        assert!(
            SCRIPT_CONTENT.contains("${RED}"),
            "test.sh missing RED color usage"
        );

        // Check for dynamic test result format with icons and colors
        assert!(
            SCRIPT_CONTENT.contains("✅") && SCRIPT_CONTENT.contains("PASSED"),
            "test.sh missing PASSED format with checkmark"
        );
        assert!(
            SCRIPT_CONTENT.contains("❌") && SCRIPT_CONTENT.contains("FAILED"),
            "test.sh missing FAILED format with X mark"
        );

        // Check for pass/fail count format (dynamic variables)
        assert!(
            SCRIPT_CONTENT.contains("passed,") && SCRIPT_CONTENT.contains("failed)"),
            "test.sh missing pass/fail count format"
        );

        // Check for overall test summary
        assert!(
            SCRIPT_CONTENT.contains("All Tests:"),
            "test.sh missing overall test summary"
        );

        // Verify the script extracts counts from cargo output
        assert!(
            SCRIPT_CONTENT.contains("grep -o '[0-9]\\+ passed'"),
            "test.sh not extracting passed test counts"
        );
        assert!(
            SCRIPT_CONTENT.contains("grep -o '[0-9]\\+ failed'"),
            "test.sh not extracting failed test counts"
        );

        // Check for proper exit codes
        assert!(
            SCRIPT_CONTENT.contains("exit 0") || SCRIPT_CONTENT.contains("exit 1"),
            "test.sh missing proper exit codes"
        );
    }
}
