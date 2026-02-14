FROM rust:1-alpine AS base
RUN apk add --no-cache build-base openssl-dev openssl-libs-static

FROM base AS builder
WORKDIR /app

# Copy manifests to cache dependencies
COPY Cargo.toml Cargo.lock ./

# Create dummy source to build dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release --locked && \
    rm -rf src

# Copy actual source code
COPY . .

# Update modification time to force rebuild of the application
RUN touch src/main.rs && \
    cargo build --release --locked

FROM alpine:latest AS runner
WORKDIR /app
COPY --from=builder /app/target/release/gemini_api_proxy .
EXPOSE 8080
ENV RUST_LOG=info
CMD [ "./gemini_api_proxy" ]
