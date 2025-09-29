#[cfg(test)]
mod test_dependency_audit_tests {
    use std::process::Command;

    #[test]
    fn test_comprehensive_security_audit() {
        run_dependency_audit();
        run_license_audit();
        run_unmaintained_audit();
    }

    fn run_dependency_audit() {
        let AUDIT_CHECK = Command::new("cargo").args(["audit", "--version"]).output();

        match AUDIT_CHECK {
            Ok(OUTPUT) if OUTPUT.status.success() => {
                let AUDIT_RESULT = Command::new("cargo")
                    .args(["audit"])
                    .output()
                    .expect("Failed to run cargo audit");

                if !AUDIT_RESULT.status.success() {
                    let STDERR = String::from_utf8_lossy(&AUDIT_RESULT.stderr);
                    let STDOUT = String::from_utf8_lossy(&AUDIT_RESULT.stdout);
                    panic!("Security vulnerabilities found:\nSTDOUT:\n{STDOUT}\nSTDERR:\n{STDERR}");
                }
            }
            _ => println!(
                "WARNING: cargo-audit not installed. Install with: cargo install cargo-audit"
            ),
        }
    }

    fn run_license_audit() {
        let AUDIT_CHECK = Command::new("cargo").args(["audit", "--version"]).output();

        if AUDIT_CHECK.is_ok() && AUDIT_CHECK.unwrap().status.success() {
            let RESULT = Command::new("cargo")
                .args(["audit", "--deny", "warnings", "--deny", "unmaintained"])
                .output()
                .expect("Failed to run license audit");

            if !RESULT.status.success() {
                let STDERR = String::from_utf8_lossy(&RESULT.stderr);
                panic!("License or maintenance issues found:\n{STDERR}");
            }
        }
    }

    fn run_unmaintained_audit() {
        let AUDIT_CHECK = Command::new("cargo").args(["audit", "--version"]).output();

        if AUDIT_CHECK.is_ok() && AUDIT_CHECK.unwrap().status.success() {
            let RESULT = Command::new("cargo")
                .args(["audit", "--stale"])
                .output()
                .expect("Failed to run stale dependency audit");

            if !RESULT.status.success() {
                let STDERR = String::from_utf8_lossy(&RESULT.stderr);
                println!("WARNING: Stale dependencies found:\n{STDERR}");
            }
        }
    }
}
