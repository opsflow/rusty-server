FROM rust:1.40 as builder

WORKDIR /app

COPY . .

RUN cargo install --path .

FROM debian:buster-slim


COPY --from=builder /usr/local/cargo/bin/rusty_server /usr/local/bin/rusty_server

# RUN mkdir www; cd www; mkdir html

COPY --from=builder /app/public/ /app/public/
ENV SERVER_ADDRESS="0.0.0.0:8080"

CMD ["rusty_server"]
