FROM rust:1.78.0-alpine3.19 as build
RUN apk add --no-cache musl-dev

COPY . /dnstp/
WORKDIR /dnstp

RUN cargo build --release

FROM alpine:3.19

COPY --from=build /dnstp/target/release/dnstp /dnstp/dnstp
WORKDIR /dnstp

EXPOSE 5353/udp

ENTRYPOINT ["/dnstp/dnstp"]
CMD ["--address", "0.0.0.0:5353"]