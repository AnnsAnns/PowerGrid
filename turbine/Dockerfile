# Build-Stage
FROM rust:latest AS builder

# Setze das Arbeitsverzeichnis
WORKDIR /app

# Kopiere die Cargo-Konfigurationsdateien und den Quellcode
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Baue die Anwendung im Release-Modus
RUN cargo build --release

# Runtime-Stage
FROM debian:bullseye-slim

# Installiere notwendige Bibliotheken
RUN apt-get update && apt-get install -y libssl-dev && rm -rf /var/lib/apt/lists/*

# Setze das Arbeitsverzeichnis
WORKDIR /app

# Kopiere das kompilierte Binary aus der Build-Stage
COPY --from=builder /app/target/release/turbine .

# Setze das Standardkommando für den Container
CMD ["./turbine"]