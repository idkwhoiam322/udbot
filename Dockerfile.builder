FROM rust:bullseye as builder

RUN mkdir /build
WORKDIR /build

CMD ["cargo", "build", "--release"]