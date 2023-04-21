.PHONY: dev
dev:
	cargo run

.PHONY: build
build:
	cargo build

.PHONY: help
help:
	cd help && python3 build_help.py