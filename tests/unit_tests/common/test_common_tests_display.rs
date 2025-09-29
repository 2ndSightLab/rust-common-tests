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

        // Check that script displays common tests with header
        assert!(
            SCRIPT_CONTENT.contains("echo \"Running common tests from rust-common-tests...\""),
            "test.sh missing common tests display header"
        );

        // Check that script changes directory to rust-common-tests
        assert!(
            SCRIPT_CONTENT.contains("cd ../rust-common-tests &&"),
            "test.sh not changing to rust-common-tests directory"
        );

        // Check that script runs specific test binaries for common tests
        assert!(
            SCRIPT_CONTENT.contains("cargo test --test integration --test security_checks --test unit_tests 2>&1"),
            "test.sh not running correct test binaries for common tests"
        );

        // Check that script colorizes common test output
        assert!(
            SCRIPT_CONTENT.contains("echo -e \"${line% ok} ${GREEN}ok${NC}\"") &&
            SCRIPT_CONTENT.contains("echo -e \"${line% FAILED} ${RED}FAILED${NC}\""),
            "test.sh not colorizing common test output"
        );

        // Check that script returns to original directory
        assert!(
            SCRIPT_CONTENT.contains("cd - > /dev/null"),
            "test.sh not returning to original directory after common tests"
        );

        // Check that common tests section comes before local tests section
        let COMMON_TESTS_POS = SCRIPT_CONTENT.find("Running common tests from rust-common-tests...");
        let LOCAL_TESTS_POS = SCRIPT_CONTENT.find("Running local tests...");
        
        assert!(
            COMMON_TESTS_POS.is_some() && LOCAL_TESTS_POS.is_some(),
            "test.sh missing common or local tests sections"
        );
        
        assert!(
            COMMON_TESTS_POS < LOCAL_TESTS_POS,
            "test.sh common tests section should come before local tests section"
        );
    }
}
