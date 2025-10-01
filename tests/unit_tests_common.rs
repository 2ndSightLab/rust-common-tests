mod common;

mod unit_tests_common {
    pub mod common {
        pub mod best_practices_test;
        pub mod config_standards_test;
        pub mod service_config_test;
        pub mod test_common_tests_display;
        pub mod test_local_tests_display;
        pub mod test_module_naming;
        pub mod test_script_validation;
        pub mod toml_lint_test;
        pub mod variable_naming_test;
    }
}

pub mod test_unit_tests_common;

#[test]
fn run_all_common_unit_tests() {
    common::run_common_tests("unit_tests_common");
}
