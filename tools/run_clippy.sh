#!/usr/bin/env bash
# Script (heavily) inspired by 'tock'
# Reference: https://github.com/tock/tock/blob/master/tools/run_clippy.sh

# Check to see if we can execute `cargo clippy`.
# We don't want to force an installation onto the user, so for we
# will only notify them of the issue.
if ! rustup component list | grep 'clippy.*(installed)' -q; then
    echo "Could not check formatting with clippy, 'clippy' must be installed!"
    exit 1	
fi

# TODO: What arguments do we want to pass to clippy?
CLIPPY_ARGS="-D warnings"

cargo clippy -- $CLIPPY_ARGS
