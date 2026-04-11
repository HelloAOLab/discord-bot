# ── Build stage ──────────────────────────────────────────────────────────────
FROM rust:1-bookworm AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libsqlite3-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy the whole workspace and build in release mode.
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/

RUN cargo build --release --bin sb-discord-bot

# ── Runtime stage ─────────────────────────────────────────────────────────────
FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && apt-get install -y \
    libssl3 \
    libsqlite3-0 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/sb-discord-bot ./sb-discord-bot

# /data is the persistent volume where the SQLite database lives.
RUN mkdir -p /data
VOLUME /data

ENV DATABASE_URL=sqlite:///data/bot.db?mode=rwc

CMD ["./sb-discord-bot"]
