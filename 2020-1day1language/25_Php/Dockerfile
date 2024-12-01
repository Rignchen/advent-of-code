FROM php:8.3.7-fpm-alpine
COPY ./src /app
COPY ./data /app/data
WORKDIR /app
ENTRYPOINT ["tail", "-f", "/dev/null"]
