# FROM rust:latest as build

# # create a new empty shell project
# RUN USER=root cargo new --bin social_axum
# WORKDIR /social_axum

# # copy over your manifests
# COPY ./Cargo.lock ./Cargo.lock
# COPY ./Cargo.toml ./Cargo.toml

# # this build step will cache your dependencies
# RUN cargo build --release
# RUN rm src/*.rs

# # copy your source tree
# COPY ./src ./src

# # build for release
# RUN rm ./target/release/deps/social_axum*
# RUN alias rust-musl-builder='docker run --rm -it -v "$(pwd)":/home/rust/src ekidd/rust-musl-builder'


# RUN cargo build --release

# # our final base
# FROM debian:buster-slim

# # copy the build artifact from the build stage
# COPY --from=build /social_axum/target/release/social_axum .

# # set the startup command to run your binary
# CMD ["./social_axum"]


ARG BASE_IMAGE=ekidd/rust-musl-builder:latest

# Our first FROM statement declares the build environment.
FROM ${BASE_IMAGE} AS builder

# Add our source code.
ADD --chown=rust:rust . ./

# Build our application.
RUN cargo build --release

# Now, we need to build our _real_ Docker container, copying in `social_axum`.
FROM alpine:latest
RUN apk --no-cache add ca-certificates
COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/social_axum \
    /usr/local/bin/
CMD /usr/local/bin/social_axum