FROM rust:latest as builder
WORKDIR /usr/src/cooktak
COPY . .
RUN cargo install --path .

FROM debian:buster
RUN apt-get update && apt-get install -y mariadb-dev
COPY --from=builder /usr/local/cargo/bin/noctis /usr/local/bin/cooktak
CMD ["cooktak"]
