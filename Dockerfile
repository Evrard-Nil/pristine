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
COPY src ./src
COPY .gh_pk ./.gh_pk

# Build the release binary
RUN cargo build --release --locked --offline || cargo build --release --locked

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
EXPOSE 5000

# Command to run the application
CMD ["./pristine_agent"]