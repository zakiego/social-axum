FROM rust:1 as build-env

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
RUN cargo build --release

# our final base
FROM gcr.io/distroless/cc-debian12
COPY --from=build-env /social_axum/target/release/social_axum .
CMD ["./hello-world-distroless"]