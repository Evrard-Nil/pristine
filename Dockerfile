# Stage 1: Builder
FROM rust:1.88-slim-bookworm AS builder

WORKDIR /app

# Install system dependencies required for git2
RUN apt-get update && apt-get install -y \
    libssl-dev \
    pkg-config \
    cmake \
    git \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./

# Compile an empty crate to cache dependencies
RUN mkdir -p src/bin && echo "fn main() {}" > src/main.rs  && echo "fn main() {}" > src/bin/run.rs
RUN cargo build --release
RUN rm -rf src/main.rs

COPY src ./src

# Build the release binary
RUN cargo build --release

# Stage 2: Runner
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies for git2
RUN apt-get update && apt-get install -y \
    libssl3 \
    git \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/run ./pristine_agent

# Expose the port the web server runs on
EXPOSE 5005

# Command to run the application
CMD ["./pristine_agent"]
