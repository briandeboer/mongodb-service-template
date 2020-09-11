#!/usr/bin/env sh

IMAGE="${REPOSITORY_URI}-test:${MD5}"

echo "Pulling docker image $IMAGE..."

docker pull $IMAGE

if [ $? -eq 0 ]
then
  echo "Docker test build exists - using..."
else
  echo "Creating docker image $IMAGE"
  docker build -t $IMAGE -f test.Dockerfile .
  docker push $IMAGE
fi
