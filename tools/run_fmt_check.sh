#!/usr/bin/env bash
# Script (heavily) inspired by 'tock'
# Reference: https://github.com/tock/tock/blob/master/tools/run_cargo_fmt.sh

let FAIL=0

# Check to see if we can execute `cargo rust`.
# We don't want to force an installation onto the user, so for we
# will only notify them of the issue.
if ! rustup component list | grep 'rustfmt.*(installed)' -q; then
    echo "Could not check formatting, 'rustfmt' must be installed!"
    exit 1
fi

if ! cargo fmt -q -- --check; then
    printf "<- Contains formatting errors!\n"
	cargo fmt -- --check || let FAIL=FAIL+1
	printf "\n"
fi

RUST_FILES_WITH_TABS="$(git grep --files-with-matches $'\t' -- '*.rs' || grep -lr --include '*.rs' $'\t' . || true)"
if [ "$RUST_FILES_WITH_TABS" != "" ]; then
    echo "ERROR: The following files contain tab characters, please use spaces instead:"
    echo "$RUST_FILES_WITH_TABS" | sed 's/^/    -> /'
    let FAIL=FAIL+1
fi

if [[ $FAIL -ne 0 ]]; then
	echo
	echo "$(tput bold)$(tput setaf 1)Formatting errors.$(tput sgr0)"
	echo "See above for details"
fi
exit $FAIL
