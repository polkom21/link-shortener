FROM rust:1.75.0 as builder
WORKDIR /usr/src/link-shortener
COPY . .
RUN cargo install --path .
CMD ["link-shortener"]