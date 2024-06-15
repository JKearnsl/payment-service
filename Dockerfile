# Build image
FROM rust:1.79.0-alpine3.20 as build

WORKDIR /usr/service

RUN apk add --no-cache musl-dev

COPY . .


RUN cargo install --path .

# Runtime image
FROM alpine:3.20.0

WORKDIR /usr/local/bin

COPY --from=build /usr/local/cargo/bin/payment-service .
COPY --from=build /usr/service/templates ./templates
COPY --from=build /usr/service/static ./static


CMD ["payment-service"]