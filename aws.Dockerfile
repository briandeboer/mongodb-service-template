FROM 981873564135.dkr.ecr.us-east-1.amazonaws.com/alpine:3.11
RUN apk update &&\
  apk add binutils

WORKDIR /app

COPY ./build/{{project-name}} ./{{project-name}}

EXPOSE 8080
# set the startup command to run your binary
# CHANGE APP NAME BELOW
CMD ["./{{project-name}}"]