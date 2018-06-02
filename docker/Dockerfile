FROM dummy.test/rust-src:latest as src

FROM rust:latest

RUN set -x \
  && apt-get -y update \
  && apt-get install -y --no-install-recommends \
    cmake \
    ninja-build \
  && rm -rf /var/lib/apt/lists/*

RUN useradd -m -U build
COPY --from=src --chown=build / /home/build/rust
COPY config.toml /home/build/rust/

USER build
WORKDIR /home/build

RUN cd /home/build/rust && ./x.py build

RUN set -x \
  && rustup toolchain link dev /home/build/rust/build/x86_64-unknown-linux-gnu/stage2/ \
  && rustup default dev

ENV CC=/home/build/rust/build/x86_64-unknown-linux-gnu/clang/bin/clang
ENV CXX=/home/build/rust/build/x86_64-unknown-linux-gnu/clang/bin/clang++
ENV AR=/home/build/rust/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-ar
