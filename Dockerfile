# Use a Rust base image
FROM rust:latest

# Set the working directory
WORKDIR /usr/src/app

# Copy the current directory contents into the container at /usr/src/app
COPY . .

# Build the Rust application
RUN cargo build --release

# Run the Rust application
CMD ["./target/release/social_axum"]
