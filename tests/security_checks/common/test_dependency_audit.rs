#[cfg(test)]
mod test_dependency_audit_tests {
    use std::process::Command;

    #[test]
    fn test_comprehensive_security_audit() {
        let OUTPUT = Command::new("./scripts/security-checks.sh")
            .output()
            .expect("Failed to execute security-checks.sh");

        let STDERR = String::from_utf8_lossy(&OUTPUT.stderr);

        assert!(
            OUTPUT.status.success(),
            "security-checks.sh failed with exit code: {}. Output: {}",
            OUTPUT.status.code().unwrap_or(-1),
            STDERR
        );
    }
}
