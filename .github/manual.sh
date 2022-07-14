#!/bin/sh

# Run GitHub workflow tests manually.
# Run this script from the project root.
# Watch the exit code
grep "run:" .github/workflows/rust.yml | awk -F ':'  '{ print $2 " \&\&" } END { print "echo \"\nALL JOBS PASSED\" && exit 0"}' | sh
