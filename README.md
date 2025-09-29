# rust-common-tests

Common tests that can be run against any Rust project to validate security, standards, and best practices.

__Building and Testing__

```bash
# Build
./scripts/build.sh

# Run tests
./scripts/test.sh

# Check best practices
./scripts/best-practices.sh

```

## Implementation

This repository provides a comprehensive test suite that validates Rust projects against security standards, coding conventions, and best practices. The tests are organized into three main categories with specific validation rules.

### Architecture

```
rust-common-tests/
├── src/                    # Library code for reusable test functions
├── tests/                  # Test implementations
│   ├── integration/        # Integration tests (4 tests)
│   ├── security_checks/    # Security validation (6 tests)
│   └── unit_tests/         # Standards and practices (8 tests)
├── scripts/               # Automation scripts
│   ├── build.sh          # Build automation
│   ├── test.sh           # Test execution with formatted output
│   ├── best-practices.sh # Comprehensive best practices validation
│   └── install.sh        # Binary installation
└── Cargo.toml            # Project configuration with explicit test paths
```

### Test Categories

#### Integration Tests (4 tests)
- **System Resource Monitoring**: Validates monitoring capabilities
- **Security Workflow Integration**: Tests security process integration
- **Graceful Shutdown Handling**: Ensures proper shutdown procedures
- **Prerequisites Check**: Validates required dependencies and setup

#### Security Checks (6 tests)
- **Code Separation**: Ensures no test code exists in production (`src/`)
- **Dependency Validation**: Validates essential dependencies are declared
- **Dependency Audit**: Comprehensive security vulnerability scanning using `cargo audit`
- **Hardcoded Secrets Detection**: Scans for potential secrets in code
- **Process Exit Validation**: Ensures proper exit handling
- **File Permissions**: Validates secure file permission settings

#### Unit Tests (8 tests)
- **Best Practices Compliance**: Runs comprehensive linting and formatting checks
- **Configuration Standards**: Validates TOML configuration standards
- **Service Configuration**: Validates `service.toml` with `SERVICE_NAME` and `LOG_PATH`
- **Script Validation**: Ensures test scripts follow required format standards
- **TOML Linting**: Validates Cargo.toml lint overrides
- **Variable Naming**: Enforces SCREAMING_SNAKE_CASE for all variables
- **Configuration File Standards**: Validates configuration file formats
- **Naming Conventions**: Ensures consistent naming across the project

**Total: 18 tests**

## Usage

### As a Git Submodule

Add as a git submodule to your Rust project:

```bash
git submodule add https://github.com/your-org/rust-common-tests.git common-tests
cd common-tests
```

### Running Tests

Execute all tests with formatted output:

```bash
./scripts/test.sh
```

Run individual test categories:

```bash
# Integration tests only
cargo test --test integration

# Security checks only
cargo test --test security_checks

# Unit tests only
cargo test --test unit_tests
```

### Build and Install

Build the project:

```bash
./scripts/build.sh
```

Install binaries (for service projects):

```bash
./scripts/install.sh
```

Run comprehensive best practices validation:

```bash
./scripts/best-practices.sh
```

### As a Library Dependency

Import in your project's `Cargo.toml`:

```toml
[dev-dependencies]
rust-common-tests = { path = "../rust-common-tests" }
```

Access test functions programmatically:

```rust
use rust_common_tests::*;

#[test]
fn my_custom_test() {
    // Use library functions for validation
}
```

## Configuration Requirements

### Mandatory Cargo.toml Settings

Your project must include the lint override for SCREAMING_SNAKE_CASE compliance:

```toml
[lints.rust]
non_snake_case = "allow"
```

### Service Configuration (Optional)

If your project is a service, create a `service.toml` in the project root:

```toml
SERVICE_NAME = "my-service"
LOG_PATH = "/var/log/my-service.log"
```

When `service.toml` exists, the service configuration test validates:
- `SERVICE_NAME` is present and non-empty
- `LOG_PATH` is present and non-empty

When `service.toml` is missing, the test displays "N/A: No service is configured" and passes.

## Service Configuration Detection

The integration and unit tests automatically detect if a service is configured by checking for `service.toml` in the project root. When no service configuration is found, tests display "N/A: No service is configured in this repository (service.toml not found)" and skip service-specific validations appropriately.

## Variable Naming Convention

All variables throughout the project (including test code) must use SCREAMING_SNAKE_CASE:

```rust
// Correct
let SERVICE_NAME = "my-service";
let CONFIG_PATH = Path::new("config.toml");

// Incorrect - will fail tests
let service_name = "my-service";
let configPath = Path::new("config.toml");
```

## Test Output Format

The test suite provides formatted output with:
- Color-coded results (✅ PASSED / ❌ FAILED)
- Individual test category summaries
- Pass/fail counts for each category
- Overall test suite status
- Detailed failure information when tests fail

## Dependencies

Required dependencies for full functionality:
- `cargo-audit` (for security auditing)
- `toml` (for configuration parsing)
- `regex` (for pattern matching)
- `walkdir` (for directory traversal)

## Integration with CI/CD

The test suite is designed for CI/CD integration:
- Non-interactive execution support
- Proper exit codes (0 for success, 1 for failure)
- Structured output for parsing
- Automatic build type selection in non-interactive environments

Example CI usage:

```bash
# In CI environment
echo "1" | ./scripts/test.sh  # Force debug build
./scripts/build.sh --release  # Release build
```

## Extending the Test Suite

### Adding New Tests

1. Create test file in appropriate category directory
2. Add module reference to the category's main test file
3. Follow SCREAMING_SNAKE_CASE naming convention
4. Update this README with new test description

### Custom Validation Rules

The test suite can be extended with project-specific validation rules while maintaining the core security and standards checks.
