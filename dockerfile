FROM rust:1.76.0-slim AS builder

WORKDIR /usr/src/app
COPY . .

RUN cargo build --release

FROM debian:bookworm-slim AS runner
WORKDIR /usr/src/app

COPY --from=builder ./usr/src/app/target/release/web-api .

EXPOSE 3000
CMD ["./web-api"]