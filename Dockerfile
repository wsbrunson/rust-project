# Rust as the base image
FROM rust:1.57 as build

# Create a new empty shell project
RUN USER=root cargo new --bin rust-project
WORKDIR /rust-project

# Copy our manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# Copy the source code
COPY ./src ./src

# Build for release.
RUN rm ./target/release/deps/rust_project*
RUN cargo build --release

# The final base image
FROM debian:buster-slim

# Copy from the previous build
COPY --from=build /rust-project/target/release/rust-project /usr/src/rust-project
# COPY --from=build /rust-project/target/release/rust-project/target/x86_64-unknown-linux-musl/release/rust-project .

# Run the binary
CMD ["/usr/src/rust-project"]
