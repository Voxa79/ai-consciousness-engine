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
COPY api-gateway/Cargo.toml ./api-gateway/

# Create dummy main.rs to build dependencies
RUN mkdir -p api-gateway/src && \
    echo "fn main() {}" > api-gateway/src/main.rs

# Build dependencies
RUN cargo build --release --bin api-gateway

# Copy source code
COPY api-gateway/src ./api-gateway/src/

# Build application
RUN touch api-gateway/src/main.rs && \
    cargo build --release --bin api-gateway

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Copy binary
COPY --from=builder /app/target/release/api-gateway /app/api-gateway

# Create non-root user
RUN useradd -r -s /bin/false appuser && \
    chown appuser:appuser /app/api-gateway

USER appuser

EXPOSE 3000

HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:3000/health || exit 1

CMD ["./api-gateway"]
