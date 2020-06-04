FROM rust:latest as builder

WORKDIR /usr/src/cooktak
COPY . .
RUN cargo install --path .

FROM debian:stable-slim

RUN apt-get update && apt-get install libmariadb-dev -y
COPY --from=builder /usr/local/cargo/bin/noctis /usr/local/bin/cooktak
CMD ["cooktak"]
