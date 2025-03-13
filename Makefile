CACHE_DIR?=.cache

## check: Run the linters
check: clippy fmt ## - Run the linter

## fix: Run the linter and make changes
fix: ## - Run linter and make the changes locally automatically
	cargo fmt --all

## clippy: Run the clippy linter
clippy: ## - Run the clippy linter
	cargo clippy --all --all-features --tests -- -D warnings


## fmt: Run the formatter
fmt: ## - Run the formatter
	cargo fmt --all --check

## test: Run the tests
test:   ## - Run the tests
	cargo test

## doc: Generate the doc
doc: ## - Run the tests
	cargo doc --no-deps --workspace

## ci-test: Run the tests in CI
ci-test: ## - Test the github workflow locally
	act -W ./.github/workflows/test-and-build.yml

## clean: Clean the project
clean: clean_cache ## - Clean the projects source, remove caches and dependencies
	cargo clean

## clean_cache: Clean the cache directory.
clean_cache: ## - clean the cache directory
	rm -rf $(CACHE_DIR)

## help: Show this help.
.PHONY: help
help: Makefile
	@printf "Usage: make [target] [VARIABLE=value]\nTargets:\n"
	@sed -n 's/^## //p' $< | awk 'BEGIN {FS = ":"}; { if(NF>1 && $$2!="") printf "  \033[36m%-25s\033[0m %s\n", $$1, $$2 ; else printf "%40s\n", $$1};'
	@printf "Variables:\n"
	@grep -E "^[A-Za-z0-9_]*\?=" $< | awk 'BEGIN {FS = "\\?="}; { printf "  \033[36m%-25s\033[0m  Default values: %s\n", $$1, $$2}'
