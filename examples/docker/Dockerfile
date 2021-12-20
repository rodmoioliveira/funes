FROM rust:latest as build
ENV PKG_CONFIG_ALLOW_CROSS=1
RUN cargo install funes

FROM gcr.io/distroless/cc
COPY --from=build /usr/local/cargo/bin/funes funes
CMD ["./funes"]
