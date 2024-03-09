FROM rust:latest as build

# create a new empty shell project
RUN USER=root cargo new --bin social_axum
WORKDIR /social_axum

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/social_axum*
RUN alias rust-musl-builder='docker run --rm -it -v "$(pwd)":/home/rust/src ekidd/rust-musl-builder'


RUN cargo build --release

# our final base
FROM alpine:latest

# copy the build artifact from the build stage
COPY --from=build /social_axum/target/release/social_axum .

# set the startup command to run your binary
CMD ["./social_axum"]