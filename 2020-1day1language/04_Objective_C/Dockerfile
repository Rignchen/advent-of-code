FROM ubuntu:24.04 AS base
RUN apt-get update && \
    apt-get install -y \
        build-essential \
        gnustep \
        gnustep-devel \
        gobjc
ENTRYPOINT ["tail", "-f", "/dev/null"]

# I split them in two stages as I use a script to build run and open a shell, but this script also remove the container and image at the end, splitting in multiple stages mean that only the last stage will be removed and the 1st is kept in cache, that way I don't have to wait for apt-get to install everything again

FROM base
WORKDIR /app
COPY ./src .
RUN mkdir bin && \
    chmod +x compile.sh && \
    ./compile.sh && \
    rm compile.sh

