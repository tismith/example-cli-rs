FROM ekidd/rust-musl-builder as build

COPY . .

RUN sudo chown -R rust:rust .

ENV PKG_CONFIG_ALLOW_CROSS=1

RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine

COPY --from=build /home/rust/src/target/x86_64-unknown-linux-musl/release/example-cli /usr/local/bin

CMD ["/usr/local/bin/example-cli", "--help"]

