#!/bin/bash

get_common_test_files() {
    local category_name="$1"
    
    # Get the common test directory path
    local common_test_dir=$(get_test_dir "common" 2>/dev/null)
    if [[ -z "$common_test_dir" || ! -d "$common_test_dir" ]]; then
        return 1
    fi
    
    # Run cargo test --list from the rust-common-tests directory to get individual tests
    cd "$common_test_dir/.." || return 1
    cargo test --test "$category_name" -- --list 2>/dev/null | grep ": test$" | sed 's/: test$//' | grep -v "run_all_common"
}

export -f get_common_test_files
