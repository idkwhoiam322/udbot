FROM fedora:34

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    RUST_VERSION=1.53.0 \
    LANG="C.UTF-8" \
    PATH=$PATH:/usr/local/cargo/bin

RUN dnf -y update && dnf -y install gcc gcc-c++ openssl-devel openssl clang wget

# Install Rust Toolchain
RUN mkdir /usr/local/rustup /usr/local/cargo && \
    wget "https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init"; \
    chmod +x rustup-init; \
    ./rustup-init -y --no-modify-path --profile default --default-toolchain $RUST_VERSION; \
    rm rustup-init; \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME;

RUN mkdir /build
WORKDIR /build

CMD ["cargo", "build", "--release"]