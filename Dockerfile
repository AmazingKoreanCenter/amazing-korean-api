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
COPY crates/crypto/Cargo.toml ./crates/crypto/Cargo.toml

# Create dummy sources for dependency caching
RUN mkdir -p src/bin && \
    echo "fn main() {}" > src/main.rs && \
    echo "fn main() {}" > src/bin/rekey_encryption.rs && \
    echo "fn main() {}" > src/bin/seed_guide.rs && \
    echo "fn main() {}" > src/bin/seed_hymn_account.rs && \
    echo "" > src/lib.rs && \
    mkdir -p crates/crypto/src && \
    echo "" > crates/crypto/src/lib.rs

# Build dependencies only (cache layer)
RUN cargo build --release && rm -rf src crates/crypto/src

# Copy actual source code
COPY crates/crypto ./crates/crypto
COPY src ./src
COPY migrations ./migrations
COPY .sqlx ./.sqlx

# Build the application
RUN touch src/main.rs src/lib.rs src/bin/rekey_encryption.rs src/bin/seed_guide.rs src/bin/seed_hymn_account.rs crates/crypto/src/lib.rs && cargo build --release

# Stage 2: Runtime
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies (curl = HEALTHCHECK 용, N-20)
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /app/target/release/amazing-korean-api /app/amazing-korean-api
# guide(온라인 콘텐츠) 시드 적재용 — 시드 파일은 커밋 금지(scp 전달), AMK_GUIDE_CONTENT_DESIGN §4
COPY --from=builder /app/target/release/seed_guide /app/seed_guide
# HYMN 시스템 계정 시드용 (콘텐츠 시딩 선행, 수동 1회 docker exec)
COPY --from=builder /app/target/release/seed_hymn_account /app/seed_hymn_account

# Copy migrations (sqlx::migrate!() 런타임 자동 실행용은 바이너리에 임베딩됨)
# 클린 배포 참조용으로 유지
COPY migrations ./migrations

# Copy seeds (클린 배포 시 시드 데이터 수동 투입용)
COPY seeds ./seeds

# Create non-root user (security N-19)
RUN useradd -r -u 1001 -M -d /nonexistent appuser \
    && chown -R appuser:appuser /app

USER appuser

# Expose port
EXPOSE 3000

# N-20: HEALTHCHECK — docker compose 의 service health 자동 검사 활성
HEALTHCHECK --interval=30s --timeout=5s --start-period=15s --retries=3 \
  CMD curl --fail --silent --max-time 3 http://localhost:3000/health || exit 1

# Run the binary
CMD ["./amazing-korean-api"]
