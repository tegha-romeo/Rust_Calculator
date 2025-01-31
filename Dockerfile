# Use the official Rust image as base
FROM rust:latest AS build

# Set the working directory
WORKDIR /app

# Copy the source code
COPY . .

# Build the Rust application
RUN cargo build 

# Use a minimal runtime image
FROM ubuntu:latest
WORKDIR /app

# Copy the compiled binary
COPY --from=build /app/target/debug/Rust_Calculator /app/rust_calculator

 # Run the calculator
CMD ["/app/rust_calculator"]
