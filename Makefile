.PHONY: lint
lint: 
	@cargo clippy --all --all-features --tests -- -D warnings

.PHONY: example
example:
	@cargo run --example example
