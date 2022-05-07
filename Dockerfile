FROM node:16 AS node
COPY package.json package-lock.json ./
RUN npm ci
COPY js js
COPY tsconfig.json webpack.config.js ./
RUN node_modules/.bin/webpack

FROM rust:1.56 AS rust
WORKDIR /build
COPY Cargo.lock Cargo.toml ./
RUN mkdir src
RUN touch src/main.rs
RUN cargo build --release || true
COPY src src
COPY templates templates
COPY --from=node entry ./
RUN cargo build --release

FROM debian:buster
RUN apt-get update \
    && apt-get install -y --no-install-recommends libpq5=11.7-0+deb10u1 libssl1.1=1.1.1d-0+deb10u2 \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*
COPY migrations migrations
COPY static static
COPY migrations languages.json ./
COPY --from=node static static
COPY --from=rust build/target/release/pastebinrun pastebinrun
ENTRYPOINT ["./pastebinrun"]
EXPOSE 8080/tcp
