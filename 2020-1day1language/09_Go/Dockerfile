FROM golang:1.22.3-alpine3.19 as base
RUN apk add --no-cache bash

FROM base
COPY ./src /app
COPY ./data /app/data
WORKDIR /app
ENTRYPOINT ["tail", "-f", "/dev/null"]