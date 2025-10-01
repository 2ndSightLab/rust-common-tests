mod common;

mod security_checks_common {
    pub mod common {
        pub mod test_code_separation;
        pub mod test_dependency_audit;
        pub mod test_dependency_validation;
        pub mod test_file_permissions;
        pub mod test_hardcoded_secrets;
        pub mod test_process_exit;
    }
}

pub mod test_security_checks_common;

#[test]
fn run_all_common_security_tests() {
    common::run_common_tests("security_checks_common");
}
