FROM rust:alpine AS build-development
RUN apk add --no-cache musl-dev
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY fb/Cargo.toml ./fb/
COPY fb-macros/Cargo.toml ./fb-macros/
RUN cargo fetch

COPY fb-macros/src ./fb-macros/src

# Dependency caching layer
RUN mkdir -p fb/src/bin \
    && echo 'fn main() {}' > fb/src/bin/main.rs \
    && echo 'fn main() {}' > fb/src/bin/migration.rs \
    && echo 'fn main() {}' > fb/src/bin/doc.rs \
    && cargo build \
    && rm -rf fb/src

COPY fb/src ./fb/src
COPY fb/migrations ./fb/migrations
RUN touch fb/src/bin/main.rs fb/src/bin/migration.rs fb/src/bin/doc.rs && cargo build

FROM rust:alpine AS build-release
RUN apk add --no-cache musl-dev
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY fb/Cargo.toml ./fb/
COPY fb-macros/Cargo.toml ./fb-macros/
RUN cargo fetch

COPY fb-macros/src ./fb-macros/src

# Dependency caching layer
RUN mkdir -p fb/src/bin \
    && echo 'fn main() {}' > fb/src/bin/main.rs \
    && echo 'fn main() {}' > fb/src/bin/migration.rs \
    && echo 'fn main() {}' > fb/src/bin/doc.rs \
    && cargo build --release \
    && rm -rf fb/src

COPY fb/src ./fb/src
COPY fb/migrations ./fb/migrations
RUN touch fb/src/bin/main.rs fb/src/bin/migration.rs fb/src/bin/doc.rs && cargo build --release

FROM rust:alpine AS build-migration
RUN apk add --no-cache musl-dev
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY fb/Cargo.toml ./fb/
COPY fb-macros/Cargo.toml ./fb-macros/
RUN cargo fetch

COPY fb-macros/src ./fb-macros/src

# Dependency caching layer
RUN mkdir -p fb/src/bin \
    && echo 'fn main() {}' > fb/src/bin/main.rs \
    && echo 'fn main() {}' > fb/src/bin/migration.rs \
    && echo 'fn main() {}' > fb/src/bin/doc.rs \
    && cargo build --bin migration \
    && rm -rf fb/src

COPY fb/src ./fb/src
COPY fb/migrations ./fb/migrations
RUN touch fb/src/bin/migration.rs && cargo build --bin migration

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
