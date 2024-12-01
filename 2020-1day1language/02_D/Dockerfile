FROM alpine:3.19
RUN apk add --no-cache gcc-gdc
WORKDIR /app
COPY ./src .
RUN mkdir bin && \
    chmod +x compile.sh && \
    ./compile.sh && \
    rm compile.sh
ENTRYPOINT ["tail", "-f", "/dev/null"]
