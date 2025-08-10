# Dockerfile for API Gateway
FROM rust:1.84 as builder

WORKDIR /app

# Copy Cargo files first for better caching
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Bust cache explicitly when needed
ARG CACHE_BUST=initial
RUN echo "Cache bust: $CACHE_BUST"

# Improve Cargo network reliability
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
ENV CARGO_NET_RETRY=5
ENV CARGO_HTTP_MULTIPLEXING=false

# Build the API gateway
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary
COPY --from=builder /app/target/release/api-gateway /usr/local/bin/api-gateway

# Expose port
EXPOSE 3000

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:3000/health || exit 1

# Run the gateway
CMD ["api-gateway"]