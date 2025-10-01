// Common tests library for Rust projects
// This library provides common security and standards tests
// that can be run against any Rust project

// Re-export all test modules from the tests directory
// This allows other projects to run these tests by importing this crate

pub use crate::tests::*;

pub mod tests {
    pub mod integration {
        pub mod common {
            include!("../tests/integration_common/common/monitoring_integration_test.rs");
            include!("../tests/integration_common/common/security_workflow_test.rs");
            include!("../tests/integration_common/common/shutdown_handling_test.rs");
            include!("../tests/integration_common/common/test_prerequisites.rs");
        }
    }

    pub mod security_checks {
        pub mod common {
            include!("../tests/security_checks_common/common/test_code_separation.rs");
            include!("../tests/security_checks_common/common/test_dependency_audit.rs");
            include!("../tests/security_checks_common/common/test_dependency_validation.rs");
            include!("../tests/security_checks_common/common/test_hardcoded_secrets.rs");
            include!("../tests/security_checks_common/common/test_process_exit.rs");
            include!("../tests/security_checks_common/common/test_file_permissions.rs");
        }
    }

    pub mod unit_tests {
        pub mod common {
            include!("../tests/unit_tests_common/common/best_practices_test.rs");
            include!("../tests/unit_tests_common/common/test_script_validation.rs");
            include!("../tests/unit_tests_common/common/toml_lint_test.rs");
            include!("../tests/unit_tests_common/common/variable_naming_test.rs");
            include!("../tests/unit_tests_common/common/config_standards_test.rs");
        }
    }
}
