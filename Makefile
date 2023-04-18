# This Makefile has been inspired by 'tock'
# Link to their Makefile here: https://github.com/tock/tock/blob/master/Makefile
# Naming scheme and conventions have been lifted from thier "Code Review" policy set.
# Reference: https://github.com/tock/tock/blob/master/doc/CodeReview.md#3-continuous-integration

.PHONY: ci-job-format
ci-job-format:
	@echo "Checking formating of source files..."
	@./tools/run_fmt_check.sh

.PHONY: ci-job-clippy
ci-job-clippy:
	@echo "Running clippy on source files..."
	@./tools/run_clippy.sh

.PHONY: ci-runner-github
ci-runner-github: ci-job-format ci-job-clippy
	@echo "Running cargo check..."
	@cargo check
