# build from standard rust alpine
FROM rust:1.43-alpine3.11

RUN apk update &&\
  apk add binutils build-base musl g++
# add ssl dependencies
RUN apk add openssl-dev

# prebuild stuff
WORKDIR /app

COPY ./Cargo.lock ./
COPY ./Cargo.toml ./Cargo.toml
RUN mkdir ./src
RUN touch ./src/lib.rs

# this build is here to help prevent full rebuilds when not needed
RUN RUSTFLAGS='-C target-feature=-crt-static' cargo build

COPY ./src ./src
RUN RUSTFLAGS='-C target-feature=-crt-static' cargo build

RUN cp ./target/debug/sample-project ./sample-project

EXPOSE 8000

# set the startup command to run your binary
# CHANGE APP NAME BELOW
CMD ["./sample-project"]