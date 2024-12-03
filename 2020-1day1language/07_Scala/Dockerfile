FROM sbtscala/scala-sbt:eclipse-temurin-alpine-22_36_1.10.0_3.4.1
COPY ./src /app/src
COPY ./data /app/data
WORKDIR /app
RUN mkdir /app/out
RUN scalac /app/src/*.scala -d /app
ENTRYPOINT ["tail", "-f", "/dev/null"]