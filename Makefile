SHELL := /bin/bash
.DEFAULT_GOAL := help

.PHONY: help verify migrate migrate-status app

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) \
		| awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-16s\033[0m %s\n", $$1, $$2}'

verify:
	cargo fmt-check
	cargo clippy --workspace --all-targets
	cargo test --workspace

watch:
	cargo run -p bootstrap --bin rust-clean-architecture

build-release:
	cargo build --release -p bootstrap --bin rust-clean-architecture

run:
	./target/release/rust-clean-architecture
