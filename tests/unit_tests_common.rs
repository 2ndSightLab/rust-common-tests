mod common;

mod unit_tests_common {
    pub mod common {
        pub mod test_best_practices_compliance;
        pub mod test_cargo_allows_screaming_snake_case;
        pub mod test_common_tests_display;
        pub mod test_local_tests_display;
        pub mod test_module_naming;
        pub mod test_no_magic_numbers;
        pub mod test_non_snake_case_override_present;
        pub mod test_screaming_snake_case_config;
        pub mod test_screaming_snake_case_variables;
        pub mod test_script_validation;
        pub mod test_service_config_validation;
    }
}

pub mod test_unit_tests_common;

#[test]
fn run_all_common_unit_tests() {
    common::run_common_tests("unit_tests_common");
}
