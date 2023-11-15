.PHONY: build run clean test lint

build:
	cargo build

lint:
	cargo fmt -- --check
	cargo clippy -- -D warnings

run: build
	cargo run

clean:
	cargo clean

test:
	cargo test -- --nocapture