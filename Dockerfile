ARG TARGET_TAG=x86_64-musl
ARG ALPINE_DIGEST=sha256:02892826401a9d18f0ea01f8a2f35d328ef039db4e1edcc45c630314a0457d5b

FROM messense/rust-musl-cross:${TARGET_TAG}

ARG CARGO_TARGET=x86_64-unknown-linux-musl

RUN apt-get update
RUN apt-get -y --no-install-recommends install cmake build-essential golang unzip wget
RUN cd /tmp \
    && mkdir protoc && cd protoc \
    && wget -q https://github.com/protocolbuffers/protobuf/releases/download/v3.6.1/protoc-3.6.1-linux-x86_64.zip \
    && unzip protoc-*.zip \
    && cp bin/* /usr/local/bin/ \
    && cp -r include/google /usr/local/include/

RUN ln -s /usr/bin/g++ /usr/bin/musl-g++

COPY . ./

RUN cargo build --release --target=${CARGO_TARGET}

FROM alpine@${ALPINE_DIGEST}

ARG CARGO_TARGET=x86_64-unknown-linux-musl

COPY --from=0 /home/rust/src/target/${CARGO_TARGET}/release/peripherio /usr/bin

ENV PERIPHERIO_DRIVER /lib/peripherio/drivers
ENV PERIPHERIO_CATEGORY /lib/peripherio/categories
ENV PERIPHERIO_HOST 0.0.0.0
ENV PERIPHERIO_PORT 57601

VOLUME ["/lib/peripherio/drivers", "/lib/peripherio/categories"]

EXPOSE 57601

CMD ["/usr/bin/peripherio"]
