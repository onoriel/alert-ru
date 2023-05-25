# Build stage
FROM rust:latest as builder

WORKDIR /usr/src/app

# Copy project files
COPY . .

# Compile the application in release mode
RUN cargo build --release

# Production stage
FROM debian:buster-slim

WORKDIR /usr/src/app

# Copy binary files from build stage
COPY --from=builder /usr/src/app/target/release/alerts .

# Run the application
CMD ["./alerts"]
