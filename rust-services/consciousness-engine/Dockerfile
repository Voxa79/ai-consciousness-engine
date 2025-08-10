# Build stage
FROM rust:1.75-slim as builder

WORKDIR /app

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy workspace files
COPY ../Cargo.toml ../Cargo.lock ./
COPY consciousness-engine/Cargo.toml ./consciousness-engine/

# Create dummy main.rs to build dependencies
RUN mkdir -p consciousness-engine/src && \
    echo "fn main() {}" > consciousness-engine/src/main.rs

# Build dependencies
RUN cargo build --release --bin consciousness-engine

# Copy source code
COPY consciousness-engine/src ./consciousness-engine/src/
COPY consciousness-engine/migrations ./consciousness-engine/migrations/

# Build application
RUN touch consciousness-engine/src/main.rs && \
    cargo build --release --bin consciousness-engine

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Copy binary
COPY --from=builder /app/target/release/consciousness-engine /app/consciousness-engine

# Create non-root user
RUN useradd -r -s /bin/false appuser && \
    chown appuser:appuser /app/consciousness-engine

USER appuser

EXPOSE 8080

HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

CMD ["./consciousness-engine"]
