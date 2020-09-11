#!/usr/bin/env sh

echo "Creating build folder if it doesn't exist"
mkdir -p /build/build

echo "Copying source files..."
cp -R /build/src /app/src
cp /build/Cargo.* /app/
cp -R /build/tests /app/tests

echo "Running test"
RUSTFLAGS='-C target-feature=-crt-static' cargo build --release --locked --no-default-features

echo "Copying release to build folder"
cp /app/target/release/sample-project /build/build/sample-project