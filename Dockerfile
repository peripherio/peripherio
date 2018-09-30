FROM ekidd/rust-musl-builder:1.28.0

RUN sudo apt-get update
RUN sudo apt-get -y --no-install-recommends install cmake build-essential golang protobuf-compiler unzip wget
RUN cd /tmp \
    && mkdir protoc && cd protoc \
    && wget -q https://github.com/protocolbuffers/protobuf/releases/download/v3.6.1/protoc-3.6.1-linux-x86_64.zip \
    && unzip protoc-*.zip \
    && sudo cp bin/* /usr/local/bin/ \
    && sudo cp -r include/google /usr/local/include/

RUN sudo ln -s /usr/bin/g++ /usr/bin/musl-g++

COPY --chown=rust:rust . ./

RUN cargo build --release

FROM alpine:3.8

RUN apk --no-cache add ca-certificates

COPY --from=0 /home/rust/src/target/x86_64-unknown-linux-musl/release/peripherio /usr/bin

ENV PERIPHERIO_HOST 0.0.0.0
ENV PERIPHERIO_PORT 50051

EXPOSE 50051

CMD ["/usr/bin/peripherio"]
