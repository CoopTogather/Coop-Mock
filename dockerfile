FROM rust:1.76.0-slim AS builder

WORKDIR /usr/src/app
COPY . .

RUN cargo build --release

FROM debian:bookworm-slim AS runner
WORKDIR /usr/src/app

COPY --from=builder ./usr/src/app/target/release/web-api .
ARG DATABASE_URL
ENV DATABASE_URL=$DATABASE_URL

EXPOSE 3033
CMD ["./web-api"]