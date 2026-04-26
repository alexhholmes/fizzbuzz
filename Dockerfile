FROM rust:alpine AS build-development
RUN apk add --no-cache musl-dev
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY fb/Cargo.toml ./fb/
COPY fb-macros ./fb-macros/
RUN cargo fetch

# Dependency caching layer
RUN mkdir -p fb/src/bin \
    && echo 'fn main() {}' > fb/src/bin/main.rs \
    && echo 'fn main() {}' > fb/src/bin/migration.rs \
    && echo 'fn main() {}' > fb/src/bin/doc.rs \
    && cargo build -p fb \
    && rm -rf fb/src

COPY fb/src ./fb/src
COPY migrations .
RUN touch fb/src/bin/main.rs && cargo build -p fb --bin fizzbuzz

FROM rust:alpine AS build-release
RUN apk add --no-cache musl-dev
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY fb/Cargo.toml ./fb/
COPY fb-macros ./fb-macros/
RUN cargo fetch

# Dependency caching layer
RUN mkdir -p fb/src/bin \
    && echo 'fn main() {}' > fb/src/bin/main.rs \
    && echo 'fn main() {}' > fb/src/bin/migration.rs \
    && echo 'fn main() {}' > fb/src/bin/doc.rs \
    && cargo build -p fb --release \
    && rm -rf fb/src

COPY fb/src ./fb/src
COPY migrations .
RUN touch fb/src/bin/main.rs && cargo build -p fb --release --bin fizzbuzz

FROM rust:alpine AS build-migration
RUN apk add --no-cache musl-dev
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY fb/Cargo.toml ./fb/
COPY fb-macros ./fb-macros/
RUN cargo fetch

# Dependency caching layer
RUN mkdir -p fb/src/bin \
    && echo 'fn main() {}' > fb/src/bin/main.rs \
    && echo 'fn main() {}' > fb/src/bin/migration.rs \
    && echo 'fn main() {}' > fb/src/bin/doc.rs \
    && cargo build -p fb \
    && rm -rf fb/src

COPY fb/src ./fb/src
COPY migrations ./migrations
RUN touch fb/src/bin/migration.rs && CARGO_MANIFEST_DIR=/app cargo build -p fb --bin migration

FROM alpine:latest AS development
WORKDIR /app
RUN adduser -D -u 1000 app && chown app /app
COPY --from=build-development --chown=app /app/target/debug/fizzbuzz .
USER app
EXPOSE 3000
CMD ["./fizzbuzz"]

FROM alpine:latest AS production
WORKDIR /app
RUN adduser -D -u 1000 app && chown app /app
COPY --from=build-release --chown=app /app/target/release/fizzbuzz .
USER app
EXPOSE 3000
CMD ["./fizzbuzz"]

FROM alpine:latest AS migration
WORKDIR /app
RUN adduser -D -u 1000 app && chown app /app
COPY --from=build-migration --chown=app /app/target/debug/migration .
USER app
CMD ["./migration"]
