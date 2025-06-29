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
RUN cargo build --release --package power_grid_factory

# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y openssl ca-certificates && apt-get clean && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/power_grid_factory power_grid_factory
COPY ./consumer/configs/slp.csv /tmp/slp.csv

CMD ["./power_grid_factory"]