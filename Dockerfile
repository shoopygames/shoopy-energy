# Stage 1: Build
FROM rust:1.89 as builder

# Install musl target
RUN rustup target add x86_64-unknown-linux-musl

# Create app directory
WORKDIR /usr/src/xmrig-energy

# Copy Cargo files first (for caching dependencies)
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies
RUN cargo build --release --target x86_64-unknown-linux-musl

# Copy full source code
COPY . .

# Build final static binary
RUN cargo build --release --target x86_64-unknown-linux-musl

WORKDIR /usr/src/xmrig-energy/target/x86_64-unknown-linux-musl/release

RUN ls -al
RUN strip xmrig-energy
RUN ls -al
# # Stage 2: Runtime
# FROM alpine:latest

# # Install ca-certificates if your app makes HTTPS requests
# RUN apk add --no-cache ca-certificates

# WORKDIR /usr/local/bin

# # Copy the static binary from builder
# COPY --from=builder /usr/src/xmrig-energy/target/x86_64-unknown-linux-musl/release/xmrig-energy .

# # Set executable
# RUN chmod +x xmrig-energy

# # Default command
# CMD ["./xmrig-energy"]
