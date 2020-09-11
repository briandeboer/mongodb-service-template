# build from standard rust alpine
FROM 981873564135.dkr.ecr.us-east-1.amazonaws.com/rust:1.43-alpine3.11 as build

RUN apk update &&\
  apk add binutils build-base musl g++
# add ssl dependencies
RUN apk add openssl-dev

# prebuild stuff
WORKDIR /app

COPY ./Cargo.lock ./
COPY ./Cargo.toml ./Cargo.before
COPY ./src ./src

# update the version to always be 1.0.0 so that the
# hash doesn't change without dependency changes
RUN cat Cargo.before | sed -E "s/^version = (.*)$/version = \"1.0.0\"/" > Cargo.toml
RUN RUSTFLAGS='-C target-feature=-crt-static' cargo build --release --no-default-features

RUN rm -rf ./src
RUN rm Cargo.toml

CMD md5sum Cargo.lock | awk '{print $1;}'