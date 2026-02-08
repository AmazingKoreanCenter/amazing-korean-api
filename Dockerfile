# Stage 1: Build
FROM rust:1.88-bookworm AS builder

WORKDIR /app

# Enable SQLx offline mode
ENV SQLX_OFFLINE=true

# Install dependencies for sqlx
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Create dummy sources for dependency caching
RUN mkdir -p src/bin && \
    echo "fn main() {}" > src/main.rs && \
    echo "fn main() {}" > src/bin/rekey_encryption.rs && \
    echo "" > src/lib.rs

# Build dependencies only (cache layer)
RUN cargo build --release && rm -rf src

# Copy actual source code
COPY src ./src
COPY migrations ./migrations
COPY .sqlx ./.sqlx

# Build the application
RUN touch src/main.rs src/lib.rs src/bin/rekey_encryption.rs && cargo build --release

# Stage 2: Runtime
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /app/target/release/amazing-korean-api /app/amazing-korean-api

# Copy migrations (if needed at runtime)
COPY migrations ./migrations

# Expose port
EXPOSE 3000

# Run the binary
CMD ["./amazing-korean-api"]
