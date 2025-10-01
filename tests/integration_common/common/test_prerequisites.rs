/// Check test prerequisites
/// # Errors
/// Returns error if basic project structure is missing
pub fn check_test_prerequisites() -> Result<(), String> {
    let CURRENT_DIR =
        std::env::current_dir().map_err(|e| format!("Failed to get current directory: {e}"))?;

    // Skip if we're in the common tests directory itself
    if CURRENT_DIR.file_name().unwrap_or_default() == "rust-common-tests" {
        return Ok(());
    }

    // Check for basic project structure
    let SRC_DIR = CURRENT_DIR.join("src");
    if !SRC_DIR.exists() {
        return Err("src directory not found".to_string());
    }

    let CARGO_PATH = CURRENT_DIR.join("Cargo.toml");
    if !CARGO_PATH.exists() {
        return Err("Cargo.toml not found".to_string());
    }

    Ok(())
}

/// Check if service config exists
/// # Panics
/// Panics if current directory cannot be determined
#[must_use]
#[allow(dead_code)]
pub fn has_service_config() -> bool {
    let CURRENT_DIR = std::env::current_dir().expect("Failed to get current directory");
    let SERVICE_CONFIG_PATH = CURRENT_DIR.join("service.toml");
    SERVICE_CONFIG_PATH.exists()
}

#[test]
fn test_prerequisites_check() {
    match check_test_prerequisites() {
        Ok(()) => println!("Prerequisites check passed"),
        Err(e) => println!("Prerequisites check failed: {e}"),
    }
}
