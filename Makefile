.PHONY: build run clean test

build:
	docker-compose build

run: build
	docker-compose up

# Stop and remove the containers
down:
	docker-compose down

# Clean up Docker containers, and the Rust target directory
clean:
	docker-compose down --rmi all
	rm -rf target/

test:
	cargo test

