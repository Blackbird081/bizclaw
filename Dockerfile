FROM rust:1.75-slim as builder

WORKDIR /app

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock ./
COPY crates/bizclaw-core/Cargo.toml crates/bizclaw-core/
COPY crates/bizclaw-payment/Cargo.toml crates/bizclaw-payment/
COPY crates/bizclaw-support/Cargo.toml crates/bizclaw-support/
COPY crates/bizclaw-accounting/Cargo.toml crates/bizclaw-accounting/
COPY crates/bizclaw-inventory/Cargo.toml crates/bizclaw-inventory/
COPY crates/bizclaw-pos/Cargo.toml crates/bizclaw-pos/
COPY crates/bizclaw-knowledge/Cargo.toml crates/bizclaw-knowledge/
COPY crates/bizclaw-skills/Cargo.toml crates/bizclaw-skills/
COPY crates/bizclaw-platform/Cargo.toml crates/bizclaw-platform/

# Create dummy source for dependency caching
RUN mkdir -p src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy actual source
COPY . .

# Build release
RUN touch src/main.rs && cargo build --release --bin bizclaw-core

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Copy binary
COPY --from=builder /app/target/release/bizclaw-core /app/bizclaw

# Create non-root user
RUN useradd -m -s /bin/bash bizclaw
USER bizclaw

# Expose port
EXPOSE 3000

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:3000/health || exit 1

# Run
CMD ["./bizclaw"]
