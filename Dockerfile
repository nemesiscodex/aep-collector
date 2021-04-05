FROM rust:1.50-slim AS sqlx
WORKDIR /app
RUN apt-get update -qq && apt-get install -y pkg-config openssl libssl-dev libpq-dev && rm -rf /var/lib/apt/lists/*
RUN cargo install sqlx-cli --no-default-features --features postgres

FROM debian:bullseye-slim 
RUN apt-get update -qq && apt-get install -y pkg-config openssl libssl-dev libpq-dev && rm -rf /var/lib/apt/lists/*
COPY --from=sqlx /usr/local/cargo/bin/sqlx /bin/sqlx
ENV WAIT=30
WORKDIR /app
RUN mkdir migrations
COPY migrations/* /app/migrations/
ADD target/release/aep-collector .
ADD run.sh /app/
CMD ["sh", "run.sh"]
