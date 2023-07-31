FROM rust:1.71.0-alpine3.18 as builder
WORKDIR /app
COPY . .
RUN [ "apk", "add", "--no-cache", "musl-dev" ]
RUN [ "cargo", "build", "--release" ]

FROM alpine:3.18
COPY --from=builder /app/target/release/bump /usr/local/bin/bump
ENTRYPOINT [ "bump" ]
