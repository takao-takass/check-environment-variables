# build applocation
FROM rust:1.82 as builder
WORKDIR /usr/src/app
COPY Cargo.toml Cargo.lock ./
COPY . .
RUN cargo build --release

RUN mkdir -p /usr/src/app/target/release/templates
COPY templates /usr/src/app/target/release/templates

# host application
FROM ubuntu:24.04
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /usr/src/app
COPY --from=builder /usr/src/app/target/release .
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000

CMD ["./check-environment-variables"]