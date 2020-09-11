#!/usr/bin/env sh

# this will compile and then create the release images
BRANCH=$(echo $CODEBUILD_WEBHOOK_TRIGGER | sed -E 's/branch\/(.*)/\1/')

echo "BRANCH is $BRANCH"

if [ "$BRANCH" != "master" ]
then
  echo "Not master branch, skipping compile"
else
  echo "Master branch - compiling and building"
  echo "*** COMPILE ***"
  echo "Attempting to pull build image tagged with $MD5"
  ./scripts/build_release.sh
  docker run -v $(pwd):/build "${REPOSITORY_URI}-build:$MD5" /build/scripts/release.sh
  echo "**** BUILD ****"
  echo Build started on `date`
  docker build -t "${REPOSITORY_URI}:latest" -f aws.Dockerfile .
  docker tag "${REPOSITORY_URI}:latest" "${REPOSITORY_URI}:${BRANCH}_${TAG}"
  echo Build completed on `date`
  echo "Pushing Docker image to ECR"
  docker push "${REPOSITORY_URI}:latest"
  docker push "${REPOSITORY_URI}:${BRANCH}_${TAG}"
  printf '{"Tag":"%s","RepositoryUri":"%s"}' $TAG $REPOSITORY_URI $PROJECT_NAME $ARTIFACT_BUCKET > build.json
fi