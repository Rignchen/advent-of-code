FROM mcr.microsoft.com/dotnet/sdk:8.0-alpine
RUN dotnet new console -lang "F#" -o /app
WORKDIR /app
COPY ./src .
ENTRYPOINT ["tail", "-f", "/dev/null"]

