mod common;

mod integration_common {
    pub mod common {
        pub mod test_graceful_shutdown_handling;
        pub mod test_prerequisites;
        pub mod test_security_workflow_integration;
        pub mod test_system_resource_monitoring;
    }
}

pub mod test_integration_common;

#[test]
fn run_all_common_integration_tests() {
    common::run_common_tests("integration_common");
}
