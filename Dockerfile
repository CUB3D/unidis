FROM rust:latest AS build

WORKDIR /home/code

RUN apt-get update -y && apt-get install -y cmake build-essential

ADD rust-toolchain.toml .
ADD ./unidis-web/templates ./unidis-web/templates
ADD unidis-web/Cargo.lock ./unidis-web/
ADD unidis-web/Cargo.toml ./unidis-web/
ADD ./libunidis ./libunidis
ADD ./unidis-web/src/ ./unidis-web/src/

RUN cd ./unidis-web && cargo build --release --all-features

FROM rust:latest

RUN apt-get update -y && apt-get install -y curl
HEALTHCHECK --interval=30s --timeout=3s CMD curl -f http://localhost:8080/ || exit 1

WORKDIR /srv

COPY --from=build /home/code/unidis-web/target/release/unidis-web /srv/unidis-web

CMD ["/srv/unidis-web"]
