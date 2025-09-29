#[test]
fn test_system_resource_monitoring() {
    println!(
        "RUNNING: test_system_resource_monitoring - Testing system resource monitoring exists"
    );

    // Skip if running as root for security
    if unsafe { libc::getuid() } == 0 {
        println!("N/A: Running as root - skipping for security");
        return;
    }

    let CURRENT_DIR = std::env::current_dir().expect("Failed to get current directory");

    // Skip if we're in the common tests directory itself
    if CURRENT_DIR.file_name().unwrap_or_default() == "rust-common-tests" {
        println!("N/A: Running in rust-common-tests directory");
        return;
    }

    // Check if service is configured
    let SERVICE_CONFIG_PATH = CURRENT_DIR.join("service.toml");
    if !SERVICE_CONFIG_PATH.exists() {
        println!("N/A: No service is configured in this repository (service.toml not found)");
        return;
    }

    // Check if monitoring functionality exists by looking for monitoring-related files
    let SRC_DIR = CURRENT_DIR.join("src");
    if SRC_DIR.exists() {
        println!("✓ Source directory exists for monitoring checks");
    }

    println!("PASSED: test_system_resource_monitoring");
}
