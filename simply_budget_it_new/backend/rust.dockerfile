
#Stage 1: Builder
FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /backend

# Caching Dependencies
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /backend/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Build application
COPY . .
RUN cargo install diesel_cli --no-default-features --features postgres && cargo build --release --bin backend

# Stage 2: Runtime
FROM debian:bookworm-slim AS runtime
WORKDIR /backend

RUN apt-get update && apt-get install -y libpq && rm -rf /var/lib/apy/lists/*

# Copy necessary files from builder stage
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel
COPY --from=builder /backend/target/release/backend /backend/local/bin

# Set the environment variables
ENV DATABASE_URL=${POSTGRES_URL}

# Set pro
ENTRYPOINT ["/backend/local/bin/backend"]
