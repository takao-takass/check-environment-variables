# build applocation
FROM rust:1.82 as builder
WORKDIR /usr/src/app
COPY Cargo.toml Cargo.lock ./
COPY . .
RUN cargo build --release

# host application
FROM debian:buster-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /usr/src/app
COPY --from=builder /usr/src/app/target/release .
EXPOSE 8000

CMD ["./check-environment-variables"]