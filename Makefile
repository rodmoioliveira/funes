#!make

changelog: ## Autogenerate CHANGELOG.md
	@docker run -t -v "$(shell pwd)":/app/ orhunp/git-cliff:latest --config cliff.toml --output CHANGELOG.md

dev: ## Start funes application
	cargo watch -x run

doc: ## Open app documentation
	@cargo doc --open

fix: ## Fix rust code
	@cargo fix --allow-dirty --allow-staged --all-features --all-targets

fmt: ## Format rust code
	@cargo fmt --all -- --check

help: ## Display this help screen
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
	awk 'BEGIN {FS = ":.*?## "}; \
	{printf "\033[36m%-25s\033[0m %s\n", $$1, $$2}' | \
	sort

lint: ## Lint rust code
	@cargo clippy --workspace --all-targets --verbose
	@cargo clippy --workspace --all-targets --verbose --all-features

tests: ## Run tests
	@cargo test

.PHONY: changelog dev doc fix fmt help tests lint
