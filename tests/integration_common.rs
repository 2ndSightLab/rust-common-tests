mod common;

mod integration_common {
    pub mod common {
        pub mod monitoring_integration_test;
        pub mod security_workflow_test;
        pub mod shutdown_handling_test;
        pub mod test_prerequisites;
    }
}

pub mod test_integration_common;

#[test]
fn run_all_common_integration_tests() {
    common::run_common_tests("integration_common");
}
