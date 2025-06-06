# Stage 1: Build the Rust application
FROM rust:latest as builder

# Set work directory
WORKDIR /app

# Pre-cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -rf src

# Copy actual source and build
COPY . .
RUN cargo build --release

# Stage 2: Runtime image
FROM debian:bullseye-slim

# Install system dependencies
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -m appuser
USER appuser

# Set work directory
WORKDIR /home/appuser

# Copy binary from build stage
COPY --from=builder /app/target/release/rustotool /usr/local/bin/rustotool

# Copy config file if needed
COPY --from=builder /app/config.yaml ./config.yaml

# Launch the app
CMD ["rustotool"]
