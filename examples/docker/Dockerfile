FROM rust:latest as build
ENV PKG_CONFIG_ALLOW_CROSS=1
WORKDIR /usr/src/app
COPY . .
RUN cargo install --path .

FROM registry.b2w.io/docker-run/gcr.io/distroless/cc-debian10:latest
COPY --from=build /usr/local/cargo/bin/docker docker
COPY --from=build /usr/src/app/mocks /mocks
CMD ["./docker"]
