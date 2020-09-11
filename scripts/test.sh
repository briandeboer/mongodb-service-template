#!/usr/bin/env sh

echo "Copying source files..."
cp -R /build/src /app/src
cp /build/Cargo.* /app/
cp -R /build/tests /app/tests

echo "Running test"
MONGO_URL=mongodb://mongo:27017 RUSTFLAGS='-C target-feature=-crt-static' cargo suity --jobs=1 -- --test-threads=1

RESULT=$?
echo "Test complete"
cp -R /app/test-results /build/test-results

if [ $RESULT -eq 0 ]
then
  echo "Tests successful"
else
  echo "Tests failed"
  exit 1
fi
