# Stage 1: Build the application
FROM rust:1 as builder
WORKDIR /app
COPY . .
RUN cargo install --path .

# Stage 2: Run the application
FROM debian:buster-slim as runner
WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /usr/local/cargo/bin/rust-rocket-app /usr/local/bin/rust-rocket-app

# Copy the templates directory
COPY src/templates ./templates

# Set environment variables for Rocket
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000

# Expose the Rocket port
EXPOSE 8000

# Command to run the application
CMD ["rust-rocket-app"]
