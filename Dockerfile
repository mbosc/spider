# Alpine Linux with musl - matches iSH environment (x86_64)
FROM --platform=linux/amd64 rust:1.75-alpine AS builder

RUN apk add --no-cache musl-dev

WORKDIR /app
COPY Cargo.toml ./
COPY src ./src

RUN cargo build --release

# Final stage with minimal Alpine to allow extraction
FROM --platform=linux/amd64 alpine:3.19
COPY --from=builder /app/target/release/spider /spider
CMD ["cat", "/spider"]
