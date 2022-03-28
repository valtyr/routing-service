FROM rust:1 as builder
WORKDIR /app

COPY ./api ./api
COPY ./cli ./cli
COPY ./mini-router ./mini-router
COPY Cargo.toml /app/Cargo.toml
COPY Cargo.lock /app/Cargo.lock

RUN cd api && cargo install --path .


FROM debian:buster-slim as runner
WORKDIR /app
RUN mkdir data
COPY data/routing-data.br /app/data/routing-data.br
COPY --from=builder /usr/local/cargo/bin/api /usr/local/bin/api
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["api"]