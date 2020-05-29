FROM rust:latest as builder
WORKDIR /usr/src/cooktak
COPY . .
RUN cargo install --path .

FROM debian:buster
RUN apt-get update && apt-get install wget
RUN wget https://cdn.mysql.com//Downloads/MySQL-5.7/libmysqlclient20_5.7.30-1debian10_amd64.deb && apt install ./libmysqlclient20_5.7.30-1debian10_amd64.deb
COPY --from=builder /usr/local/cargo/bin/noctis /usr/local/bin/cooktak
CMD ["cooktak"]
