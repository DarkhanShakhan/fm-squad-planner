# Build stage
FROM rust:1.91-slim as builder

WORKDIR /app

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml ./
COPY backend/Cargo.toml ./backend/
COPY shared/Cargo.toml ./shared/
COPY frontend/Cargo.toml ./frontend/

# Create dummy source files to build dependencies
RUN mkdir -p backend/src shared/src frontend/src && \
    echo "fn main() {}" > backend/src/main.rs && \
    echo "pub fn add(a: i32, b: i32) -> i32 { a + b }" > shared/src/lib.rs && \
    echo "fn main() {}" > frontend/src/main.rs

# Build dependencies only
RUN cargo build --release --package backend

# Remove dummy source
RUN rm -rf backend/src shared/src frontend/src

# Copy actual source code
COPY backend/src ./backend/src
COPY shared/src ./shared/src

# Build application
RUN cargo build --release --package backend

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /app/target/release/backend /app/backend

# Create data directory for SQLite
RUN mkdir -p /app/data

EXPOSE 8080

ENV RUST_LOG=info
ENV DATABASE_URL=sqlite:///app/data/squad_planner.db

CMD ["/app/backend"]
