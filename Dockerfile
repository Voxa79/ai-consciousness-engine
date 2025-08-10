# Multi-stage Dockerfile for Complete Consciousness Platform
# Optimized for production with consciousness-engine + API gateway + Web UI

# Build stage for Rust components
FROM rust:1.75-slim as rust-builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace configuration
COPY Cargo.toml Cargo.lock ./

# Copy all Rust projects
COPY consciousness-engine/ consciousness-engine/
COPY api-gateway/ api-gateway/
COPY shared/ shared/

# Build all Rust components in release mode
RUN cargo build --release

# Build stage for Node.js/React UI
FROM node:18-alpine as node-builder

WORKDIR /app

# Copy package files
COPY web-ui/package*.json ./

# Install dependencies
RUN npm ci --only=production

# Copy source code
COPY web-ui/ .

# Build React application
RUN npm run build

# Final runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libpq5 \
    nginx \
    supervisor \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean

# Create app user
RUN useradd -m -u 1001 consciousness

# Create directory structure
RUN mkdir -p /app/{bin,web,config,data,logs} \
    && chown -R consciousness:consciousness /app

# Copy Rust binaries
COPY --from=rust-builder --chown=consciousness:consciousness /app/target/release/consciousness-engine /app/bin/
COPY --from=rust-builder --chown=consciousness:consciousness /app/target/release/api-gateway /app/bin/

# Copy React build
COPY --from=node-builder --chown=consciousness:consciousness /app/build/ /app/web/

# Copy configuration files
COPY --chown=consciousness:consciousness config/ /app/config/
COPY --chown=consciousness:consciousness docker/nginx.conf /etc/nginx/nginx.conf
COPY --chown=consciousness:consciousness docker/supervisord.conf /etc/supervisor/conf.d/supervisord.conf

# Set up nginx for serving React app and proxying API
RUN mkdir -p /var/log/nginx \
    && chown -R consciousness:consciousness /var/log/nginx \
    && chown -R consciousness:consciousness /var/lib/nginx

# Health check script
COPY --chown=consciousness:consciousness docker/health-check.sh /app/bin/health-check.sh
RUN chmod +x /app/bin/health-check.sh

# Switch to app user
USER consciousness

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=30s --retries=3 \
    CMD /app/bin/health-check.sh || exit 1

# Environment variables
ENV RUST_LOG=info
ENV CONSCIOUSNESS_CONFIG_PATH=/app/config
ENV CONSCIOUSNESS_DATA_PATH=/app/data
ENV CONSCIOUSNESS_LOG_PATH=/app/logs
ENV NODE_ENV=production

# Expose ports
EXPOSE 80 8080 8081

# Use supervisor to manage multiple processes
CMD ["/usr/bin/supervisord", "-c", "/etc/supervisor/conf.d/supervisord.conf"]

# Labels
LABEL maintainer="Plateforme d'Agents IA Ultime Team"
LABEL version="0.1.0"
LABEL description="Complete Consciousness Platform - AI Consciousness + API + Web UI"
LABEL org.opencontainers.image.source="https://github.com/your-org/consciousness-platform"