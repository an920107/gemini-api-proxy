FROM rust:1-alpine AS base
RUN apk add --no-cache build-base openssl-dev openssl-libs-static

FROM base AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM alpine:latest AS runner
WORKDIR /app
COPY --from=builder /app/target/release/gemini_api_proxy .
EXPOSE 8080
ENV RUST_LOG=info
CMD [ "./gemini_api_proxy" ]
