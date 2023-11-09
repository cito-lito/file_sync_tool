
# Use the official Rust image as a builder stage to compile the binary.
FROM rust:1.62 as builder

# Create a new empty shell project.
WORKDIR /usr/src/myapp
COPY . .

# Build the project and include all dependencies.
RUN cargo build --release

# Start a new stage from a slim version of Debian to create a smaller final image.
FROM debian:buster-slim

# Copy the binary from the builder stage to the current stage.
COPY --from=builder /usr/src/myapp/target/release/file_sync_tool /usr/local/bin/file_sync_tool

# When the container starts, run the binary.
ENTRYPOINT ["/usr/local/bin/file_sync_tool"]
