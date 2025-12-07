# Build stage
FROM rust:1.91-slim as builder

WORKDIR /app

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    wget \
    && rm -rf /var/lib/apt/lists/*

# Install wasm target and trunk
RUN rustup target add wasm32-unknown-unknown && \
    cargo install trunk --locked

# Copy manifests
COPY Cargo.toml ./
COPY backend/Cargo.toml ./backend/
COPY shared/Cargo.toml ./shared/
COPY frontend/Cargo.toml ./frontend/

# Copy frontend source and build
COPY frontend ./frontend
COPY shared ./shared

# Build frontend
WORKDIR /app/frontend
RUN trunk build --release

# Runtime stage - serve static files with nginx
FROM nginx:alpine

# Copy built files from builder
COPY --from=builder /app/frontend/dist /usr/share/nginx/html

# Copy nginx configuration
COPY frontend/nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
