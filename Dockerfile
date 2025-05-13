# Build stage
FROM rust:1.86.0-slim as builder

WORKDIR /usr/src/rilot

# Copy only the necessary files for building
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Install build dependencies and musl target
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev musl-tools && \
    rustup target add x86_64-unknown-linux-musl && \
    rm -rf /var/lib/apt/lists/*

# Build the application for musl
RUN cargo build --release --target x86_64-unknown-linux-musl

# Runtime stage
FROM alpine:3.19

WORKDIR /app

# Install runtime dependencies
RUN apk add --no-cache ca-certificates openssl

# Copy the binary from builder to a directory in PATH
COPY --from=builder /usr/src/rilot/target/x86_64-unknown-linux-musl/release/rilot /usr/local/bin/

# Create directories for config and wasm
RUN mkdir -p /app/config /app/wasm

# Create a non-root user
RUN adduser -D -u 1000 rilot
USER rilot

# Expose the port
EXPOSE 8080

# No CMD or ENTRYPOINT - let users specify how to run it