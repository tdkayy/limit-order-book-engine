# Stage 1: Build the application
FROM rust:latest AS builder

WORKDIR /usr/src/app
COPY . .

RUN apt-get update && apt-get install -y pkg-config libssl-dev

# Build the release binary
RUN cargo build --release

# Stage 2: Create the runtime image
FROM debian:bookworm-slim

# Install OpenSSL
RUN apt-get update && apt-get install -y libssl-dev && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /usr/src/app/target/release/limit-order-book-engine /usr/local/bin/limit-order-book-engine

# Copy the benchmark binary so users can verify performance
COPY --from=builder /usr/src/app/target/release/manual_benchmark /usr/local/bin/manual_benchmark

EXPOSE 4000

CMD ["limit-order-book-engine"]