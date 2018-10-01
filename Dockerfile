ARG TARGET_TAG=x86
ARG BASE_DIGEST=sha256:a8c1702fe60da76824a7604ae3a3f1db29262b9099d3a759e169a90cb90ef9e3

FROM posborne/rust-cross:${TARGET_TAG}

ARG CARGO_TARGET=x86_64-unknown-linux-gnu

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

FROM debian@${BASE_DIGEST}

ARG CARGO_TARGET=x86_64-unknown-linux-gnu

COPY --from=0 /home/rust/src/target/${CARGO_TARGET}/release/peripherio /usr/bin

ENV PERIPHERIO_DRIVER_PATH /lib/peripherio/drivers
ENV PERIPHERIO_CATEGORY_PATH /lib/peripherio/categories
ENV PERIPHERIO_HOST 0.0.0.0
ENV PERIPHERIO_PORT 57601

VOLUME ["/lib/peripherio/drivers", "/lib/peripherio/categories"]

EXPOSE 57601

CMD ["/usr/bin/peripherio"]
