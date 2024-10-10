FROM lukemathwalker/cargo-chef:latest-rust-1.81.0 AS chef

FROM chef AS plan
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS build
COPY --from=plan /recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=build ./target/release/backend_driver .
COPY --from=build .env .

ENTRYPOINT ["./backend_driver"]
