FROM alpine:3.19
RUN apk add gcc-gdc
COPY ./src /app
WORKDIR /app
RUN chmod +x compile.sh && \
    ./compile.sh && \
    rm compile.sh
ENTRYPOINT ["tail", "-f", "/dev/null"]
