ARG BASE_IMAGE=rustlang/rust:nightly
FROM ${BASE_IMAGE} AS base
WORKDIR /src
COPY backend ./backend/
COPY entity ./entity/
COPY extra_migrations ./extra_migrations/
COPY migration ./migration/
COPY shared ./shared/
COPY src ./src/
COPY tests ./tests/
COPY Cargo.lock Cargo.toml ./

FROM base AS build-server
RUN cargo build --release

FROM debian:bookworm-slim as prod
RUN apt-get update && apt install -y openssl
COPY --from=build-server /src/target/release/ticketing /bin/
ENTRYPOINT [ "/bin/ticketing" ]
