.PHONY: build run clean test lint

build:
	cargo build

lint:
	cargo clippy -- -D warnings
	cargo fmt -- --check

run: build
	cargo run

clean:
	cargo clean

test:
	cargo test -- --nocapture