FROM rust:1.28.0-slim-stretch

COPY . /build

WORKDIR /tmp
RUN apt-get update \
    && apt-get -y --no-install-recommends install cmake build-essential golang protobuf-compiler unzip wget \
    && cd /tmp \
    && wget -q https://github.com/protocolbuffers/protobuf/releases/download/v3.6.1/protoc-3.6.1-linux-x86_64.zip \
    && unzip protoc-*.zip \
    && cp bin/* /usr/local/bin/ \
    && cp -r include/google /usr/local/include/ \
    && cd /build && cargo build --release \
    && cp /build/target/release/peripherio /usr/bin \
    && cd / && rm /tmp/* -rf \
    && apt-get -y remove cmake protobuf-compiler unzip wget build-essential \
    && apt-get autoremove -y \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

ENV PERIPHERIO_HOST 0.0.0.0
ENV PERIPHERIO_PORT 50051

EXPOSE 50051

CMD ["/usr/bin/peripherio"]
