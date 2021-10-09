FROM rust:alpine as builder
LABEL stage=builder

WORKDIR /my_rust_app
COPY . /my_rust_app

RUN apk add --update gcc musl-dev
RUN cargo build --release

FROM alpine

COPY --from=builder /my_rust_app/target/release/my_rust_app /usr/sbin/my_rust_app

RUN apk add --update redis

COPY ./start.sh .

# run app
CMD redis-server
