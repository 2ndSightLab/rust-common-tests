#!/bin/bash

echo "Running Rust Best Practices Check"
echo "================================="

# Ask user for build type
echo "Select build type:"
echo "1) Debug"
echo "2) Release"
read -p "Enter choice (1 or 2): " choice

case $choice in
    1)
        BUILD_TYPE="debug"
        BUILD_FLAG=""
        ;;
    2)
        BUILD_TYPE="release"
        BUILD_FLAG="--release"
        ;;
    *)
        echo "Invalid choice. Defaulting to Debug."
        BUILD_TYPE="debug"
        BUILD_FLAG=""
        ;;
esac

echo "Using $BUILD_TYPE build..."

# Exit on any error
set -e

echo "1. Code formatting check..."
cargo fmt --check

echo "2. Clippy linting (all levels) - DENY ALL WARNINGS except naming..."
cargo clippy --all-targets --all-features -- -D warnings -W clippy::all -W clippy::pedantic -W clippy::nursery -A non_snake_case -A clippy::upper_case_acronyms

echo "3. Documentation generation..."
RUSTDOCFLAGS="-D warnings -A non_snake_case -A clippy::upper_case_acronyms" cargo doc --no-deps --document-private-items

echo "4. Dead code detection - DENY ALL WARNINGS except naming..."
RUSTFLAGS="-D warnings -A non_snake_case -A clippy::upper_case_acronyms" cargo check

echo "5. Dependency tree analysis..."
cargo tree --duplicates

echo "6. Binary/Library size analysis..."
RUSTFLAGS="-D warnings -A non_snake_case -A clippy::upper_case_acronyms" cargo build --release

# Check if this is a binary or library project
BINARY_NAME=$(grep -A 10 "^\[\[bin\]\]" Cargo.toml | grep "^name" | head -1 | sed 's/name = "\(.*\)"/\1/' | tr -d '"')
if [ -n "$BINARY_NAME" ]; then
    # Binary project
    echo "Binary size:"
    ls -lh target/release/$BINARY_NAME
else
    # Library project
    PACKAGE_NAME=$(grep "^name" Cargo.toml | head -1 | sed 's/name = "\(.*\)"/\1/' | tr -d '"' | tr '-' '_')
    echo "Library size:"
    find target/release -name "lib${PACKAGE_NAME}*" -type f | head -1 | xargs ls -lh 2>/dev/null || echo "Library file not found (this is normal for some library types)"
fi

echo "7. Architecture-specific check..."
CURRENT_ARCH=$(rustc --version --verbose | grep host | cut -d' ' -f2)
echo "Running checks for current architecture: $CURRENT_ARCH"
RUSTFLAGS="-D warnings -A non_snake_case -A clippy::upper_case_acronyms" cargo check --target $CURRENT_ARCH

echo "All essential best practices checks completed successfully!"
