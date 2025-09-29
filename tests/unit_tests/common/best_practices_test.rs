#[cfg(test)]
mod best_practices_tests {
    use std::process::Command;

    #[test]
    fn test_best_practices_compliance() {
        let OUTPUT = Command::new("./scripts/best-practices.sh")
            .output()
            .expect("Failed to execute best-practices.sh");

        let STDERR = String::from_utf8_lossy(&OUTPUT.stderr);

        assert!(
            OUTPUT.status.success(),
            "best-practices.sh failed with exit code: {}. Output: {}",
            OUTPUT.status.code().unwrap_or(-1),
            STDERR
        );

        // Only fail on actual clippy warnings (lines that start with "warning:" and contain file paths)
        let CLIPPY_WARNINGS: Vec<&str> = STDERR
            .lines()
            .filter(|line| line.trim().starts_with("warning:") && line.contains(" --> "))
            .collect();

        assert!(
            CLIPPY_WARNINGS.is_empty(),
            "Best practices check found clippy warnings:\n{}",
            CLIPPY_WARNINGS.join("\n")
        );
    }
}
