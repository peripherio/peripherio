ARG TARGET_TAG=x86_64-musl

FROM messense/rust-musl-cross:${TARGET_TAG}

ARG CARGO_TARGET=x86_64-unknown-linux-musl

RUN apt-get update
RUN apt-get -y --no-install-recommends install cmake build-essential golang protobuf-compiler unzip wget
RUN cd /tmp \
    && mkdir protoc && cd protoc \
    && wget -q https://github.com/protocolbuffers/protobuf/releases/download/v3.6.1/protoc-3.6.1-linux-x86_64.zip \
    && unzip protoc-*.zip \
    && cp bin/* /usr/local/bin/ \
    && cp -r include/google /usr/local/include/

RUN ln -s /usr/bin/g++ /usr/bin/musl-g++

COPY . ./

RUN cargo build --release --target=${CARGO_TARGET}

FROM alpine:3.8

ARG CARGO_TARGET=x86_64-unknown-linux-musl

COPY --from=0 /home/rust/src/target/${CARGO_TARGET}/release/peripherio /usr/bin

ENV PERIPHERIO_HOST 0.0.0.0
ENV PERIPHERIO_PORT 50051

EXPOSE 50051

CMD ["/usr/bin/peripherio"]
