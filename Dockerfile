ARG RUST_VERSION=1.85

FROM rust:${RUST_VERSION} AS planner
LABEL authors="zhikh"

WORKDIR /usr/src/bot

RUN cargo install cargo-chef

COPY . .

RUN cargo chef prepare --recipe-path recipe.json

FROM rust:${RUST_VERSION} AS cacher

WORKDIR /usr/src/bot

RUN cargo install cargo-chef

COPY --from=planner /usr/src/bot/recipe.json ./

RUN cargo chef cook --release --recipe-path recipe.json

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim AS runtime

ARG PROJECT_NAME

WORKDIR /usr/src/bot

RUN apt-get update && \
    apt-get install -y ca-certificates openssl && \
    rm -rf /var/lib/apt/lists/*

COPY --from=cacher /usr/src/bot/target/release/${PROJECT_NAME} /usr/local/bin/app

CMD [ "/usr/local/bin/app" ]
