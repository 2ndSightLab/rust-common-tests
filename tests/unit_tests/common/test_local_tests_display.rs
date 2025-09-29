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

        // Check that script displays local tests with header
        assert!(
            SCRIPT_CONTENT.contains("echo \"Running local tests...\""),
            "test.sh missing local tests display header"
        );

        // Check that script runs integration tests for local tests
        assert!(
            SCRIPT_CONTENT.contains("cargo test --test integration 2>&1 | while read -r line; do"),
            "test.sh not running integration tests for local tests display"
        );

        // Check that script colorizes local test output
        assert!(
            SCRIPT_CONTENT.contains("echo -e \"${line% ok} ${GREEN}ok${NC}\"") &&
            SCRIPT_CONTENT.contains("echo -e \"${line% FAILED} ${RED}FAILED${NC}\""),
            "test.sh not colorizing local test output"
        );

        // Check that local tests section comes after common tests section
        let COMMON_TESTS_POS = SCRIPT_CONTENT.find("Running common tests from rust-common-tests...");
        let LOCAL_TESTS_POS = SCRIPT_CONTENT.find("Running local tests...");
        
        assert!(
            COMMON_TESTS_POS.is_some() && LOCAL_TESTS_POS.is_some(),
            "test.sh missing common or local tests sections"
        );
        
        assert!(
            LOCAL_TESTS_POS > COMMON_TESTS_POS,
            "test.sh local tests section should come after common tests section"
        );
    }
}
