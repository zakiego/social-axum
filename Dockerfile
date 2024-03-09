# build stage
FROM rust:latest

RUN USER=root cargo new --bin social_axum
WORKDIR /social_axum

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/social_axum*
RUN cargo build --release

# final stage

FROM rust:1.49

COPY --from=build /holodeck/target/release/social_axum .
CMD ["./social_axum"]
