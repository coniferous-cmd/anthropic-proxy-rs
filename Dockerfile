# syntax=docker/dockerfile:1

# ---------------------------------------------------------------------------
# chef — shared build environment with cargo-chef installed.
# Alpine/musl build toolchain: `ring` (the rustls backend) needs a C compiler
# and perl, so build-base + perl are required, not optional.
# ---------------------------------------------------------------------------
FROM rust:1-alpine AS chef
RUN apk add --no-cache build-base perl \
    && cargo install cargo-chef --locked
WORKDIR /build

# ---------------------------------------------------------------------------
# planner — resolves dependencies into a recipe.json so the expensive
# dependency build below is cached independently of source changes.
# ---------------------------------------------------------------------------
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# ---------------------------------------------------------------------------
# builder — cook dependencies from the recipe, then build the crate itself.
# The target dir is a cache mount, so it is not part of the image layer; the
# binary is copied out to a normal path inside the same RUN.
# ---------------------------------------------------------------------------
FROM chef AS builder
COPY --from=planner /build/recipe.json recipe.json
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/build/target \
    cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/build/target \
    cargo build --release --locked \
    && cp target/release/anthropic-proxy /usr/local/bin/anthropic-proxy

# ---------------------------------------------------------------------------
# runtime — minimal Alpine image, unprivileged user.
# ---------------------------------------------------------------------------
FROM alpine:3.21

RUN apk add --no-cache ca-certificates \
    && addgroup -S app \
    && adduser -S -G app -H -s /sbin/nologin app

COPY --from=builder /usr/local/bin/anthropic-proxy /usr/local/bin/anthropic-proxy

USER app
EXPOSE 3000

# busybox wget ships with Alpine; hits the proxy's own /health endpoint.
# Shell form so PORT overrides are honoured at runtime.
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD wget -qO- "http://127.0.0.1:${PORT:-3000}/health" || exit 1

ENTRYPOINT ["anthropic-proxy"]
