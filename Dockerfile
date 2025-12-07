# Build stage
FROM rust:alpine AS builder

WORKDIR /usr/src/app

# Install build dependencies
RUN apk add --no-cache \
    musl-dev \
    pkgconfig \
    openssl-dev \
    gtk+3.0-dev \
    build-base

COPY . .

# Build the application in release mode
RUN cargo build --release

# Runtime stage
FROM alpine:latest

WORKDIR /app

# Install runtime dependencies
RUN apk add --no-cache \
    ca-certificates \
    gtk+3.0 \
    glib \
    libgcc

# Copy the binary from the builder stage
COPY --from=builder /usr/src/app/target/release/volumetrik .

# Copy static files
COPY --from=builder /usr/src/app/static ./static

# Create a directory for settings
RUN mkdir -p settings

# Expose the port
EXPOSE 8080

# Run the application
CMD ["./volumetrik"]
