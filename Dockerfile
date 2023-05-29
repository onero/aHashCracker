# Start from a Rust base image
FROM rust:1.67 as builder

# Create a new directory in the builder container and set it as the working directory
WORKDIR /aHashCracker

# Copy your source code into the container
COPY . .

# Build your application
RUN cargo build --release

# Start a new stage. This will reduce the final image size
FROM debian:buster-slim

# Create a new directory in the builder container and set it as the working directory
WORKDIR /aHashCracker

# Copy the binary from the builder stage
COPY --from=builder /aHashCracker/target/release/a_hash_cracker /usr/local/bin

# Copy the wordlist from the builder stage
COPY --from=builder /aHashCracker/wordlist .

# Run your application when the container starts
CMD ["a_hash_cracker"]
