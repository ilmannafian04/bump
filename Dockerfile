FROM node:20.4.0-alpine3.18 as ui-builder
WORKDIR /app
COPY . .
WORKDIR /app/ui
RUN [ "npm", "install", "-g", "pnpm" ]
RUN [ "pnpm", "install", "--frozen-lockfile" ]
RUN [ "pnpm", "run", "build" ]

FROM rust:1.71.1-alpine3.18 as api-builder
WORKDIR /app
COPY . .
COPY --from=ui-builder /app/ui/build /app/ui/build
RUN [ "apk", "add", "--no-cache", "musl-dev" ]
RUN [ "cargo", "build", "--release" ]

FROM alpine:3.18
COPY --from=api-builder /app/target/release/bump /usr/local/bin/bump
ENTRYPOINT [ "bump" ]
