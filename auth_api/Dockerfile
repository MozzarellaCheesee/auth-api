FROM rust:latest

WORKDIR /app
COPY . .

RUN cargo build --release

EXPOSE 5002

CMD ["./target/release/auth_api"]
