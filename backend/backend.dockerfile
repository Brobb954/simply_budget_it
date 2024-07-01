# Stage 1: Builder
FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin backend

# Install and set up diesel then remove cli
RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apy/lists/*
RUN cargo install diesel_cli --no-default-features --features postgres
RUN diesel setup

# Stage 2: Runtime
FROM debian:bookworm-slim AS runtime
WORKDIR /app
# Install libpq in the runtime image
RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/backend /usr/local/bin
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin

EXPOSE 8000

ENTRYPOINT ["/usr/local/bin/backend"]