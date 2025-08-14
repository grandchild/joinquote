#!/bin/bash

cargo build --release

test_case() {
    local description="$1"
    local expected="$2"
    shift 2
    local flags="$*"
    diff <( echo "$expected" ) \
        <( seq 5 | ./target/release/joinquote "$@" ) \
        && echo -n ✔ || echo -n ✘
    echo " $description${flags:+ ($flags)}"
}

test_case "Default" "\"1\", \"2\", \"3\", \"4\", \"5\""
test_case "No quotes" "1, 2, 3, 4, 5" -Q
test_case "Custom separator" "\"1\" | \"2\" | \"3\" | \"4\" | \"5\"" " | "
test_case "Single quotes" "'1', '2', '3', '4', '5'" -q "'"
test_case "Custom separator & no quotes" "1 -> 2 -> 3 -> 4 -> 5" " -> " -Q
test_case "Parentheses" "(1), (2), (3), (4), (5)" -q "("
test_case "Brackets" "[1], [2], [3], [4], [5]" -q "]"
test_case "Braces & separator" "{1} | {2} | {3} | {4} | {5}" " | " -q "{}"
