#!/usr/bin/env sh
IMAGE="${REPOSITORY_URI}-build:${MD5}"

echo "Pulling docker image $IMAGE..."

docker pull $IMAGE

if [ $? -eq 0 ]
then
  echo "Docker release builder exists - using..."
else
  echo "Creating docker image $IMAGE"
  docker build -t $IMAGE -f release.Dockerfile .
  docker push $IMAGE
fi
