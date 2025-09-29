#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m' # No Color

echo "Running Rust Service Test Suite"
echo "==============================="

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

# Build first
echo "Building debug binaries..."
./scripts/build.sh < /dev/null

echo "Installing debug binaries..."
./scripts/install.sh < /dev/null

echo "Running all tests..."

# Define test files
TEST_FILES=("integration" "security_checks" "unit_tests")

# Run each test file
for TEST_FILE in "${TEST_FILES[@]}"; do
    if [[ -n "$TEST_FILE" ]]; then
        echo ""
        echo "Running ${TEST_FILE} tests..."
        
        if [[ "$TEST_FILE" == "integration" ]]; then
            timeout 60s cargo test --test "$TEST_FILE" -- --test-threads=1 2>&1 | while read -r line; do
                if [[ "$line" == *"... ok" ]]; then
                    echo -e "${line% ok} ${GREEN}ok${NC}"
                elif [[ "$line" == *"... FAILED" ]]; then
                    echo -e "${line% FAILED} ${RED}FAILED${NC}"
                else
                    echo "$line"
                fi
            done
        else
            cargo test --test "$TEST_FILE" 2>&1 | while read -r line; do
                if [[ "$line" == *"... ok" ]]; then
                    echo -e "${line% ok} ${GREEN}ok${NC}"
                elif [[ "$line" == *"... FAILED" ]]; then
                    echo -e "${line% FAILED} ${RED}FAILED${NC}"
                else
                    echo "$line"
                fi
            done
        fi
    fi
done

# Calculate totals dynamically
TOTAL_PASSED=0
TOTAL_FAILED=0

echo ""
echo "Test Results Summary:"
echo "===================="

# Calculate results for each test category
for TEST_FILE in "${TEST_FILES[@]}"; do
    if [[ -n "$TEST_FILE" ]]; then
        if [[ "$TEST_FILE" == "integration" ]]; then
            TEST_OUTPUT=$(timeout 60s cargo test --test "$TEST_FILE" -- --test-threads=1 2>&1)
        else
            TEST_OUTPUT=$(cargo test --test "$TEST_FILE" 2>&1)
        fi
        TEST_PASSED=$(echo "$TEST_OUTPUT" | grep -o '[0-9]\+ passed' | head -1 | grep -o '[0-9]\+' || echo "0")
        TEST_FAILED=$(echo "$TEST_OUTPUT" | grep -o '[0-9]\+ failed' | head -1 | grep -o '[0-9]\+' || echo "0")
        
        # Display result for test categories
        TOTAL_PASSED=$((TOTAL_PASSED + TEST_PASSED))
        TOTAL_FAILED=$((TOTAL_FAILED + TEST_FAILED))
        
        TEST_NAME=$(echo "${TEST_FILE}" | sed 's/_/ /g' | sed 's/\b\w/\U&/g')
        if [[ $TEST_FAILED -eq 0 ]]; then
            echo -e "✅ ${TEST_NAME}: ${GREEN}PASSED${NC} ($TEST_PASSED passed, $TEST_FAILED failed)"
        else
            echo -e "❌ ${TEST_NAME}: ${RED}FAILED${NC} ($TEST_PASSED passed, $TEST_FAILED failed)"
        fi
    fi
done

echo ""

# Overall result
if [ $TOTAL_FAILED -eq 0 ]; then
    echo -e "✅ All Tests: ${GREEN}PASSED${NC} ($TOTAL_PASSED passed, $TOTAL_FAILED failed)"
    exit 0
else
    echo -e "❌ All Tests: ${RED}FAILED${NC} ($TOTAL_PASSED passed, $TOTAL_FAILED failed)"
    exit 1
fi
