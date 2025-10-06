#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_service_config_validation() {
        let SERVICE_TOML_PATH = Path::new("../service.toml");

        if !SERVICE_TOML_PATH.exists() {
            println!("N/A: No service is configured in this repository (service.toml not found)");
            return;
        }

        let CONTENT = fs::read_to_string(SERVICE_TOML_PATH).expect("Failed to read service.toml");

        let TOML_VALUE: toml::Value =
            toml::from_str(&CONTENT).expect("Failed to parse service.toml");

        let SERVICE_NAME = TOML_VALUE
            .get("SERVICE_NAME")
            .expect("SERVICE_NAME not found in service.toml");

        assert!(
            !SERVICE_NAME.as_str().unwrap_or("").is_empty(),
            "SERVICE_NAME must be set to a non-empty value"
        );

        let LOG_PATH = TOML_VALUE
            .get("LOG_PATH")
            .expect("LOG_PATH not found in service.toml");

        assert!(
            !LOG_PATH.as_str().unwrap_or("").is_empty(),
            "LOG_PATH must be set to a non-empty value"
        );
    }
}
