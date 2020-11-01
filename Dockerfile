FROM rust:1.47.0 as build-env
WORKDIR /app
ADD . /app  
RUN cargo build --release

FROM gcr.io/distroless/cc as final
COPY --from=build-env /app/target/release/rust-actix-web /
CMD ["./rust-actix-web"]
