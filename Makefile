.PHONY: build run clean test

build:
	cargo build
	cargo clippy
	cargo fmt

run: build
	cargo run

clean:
	rm -rf target/

test:
	cargo test

