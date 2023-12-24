FROM mcr.microsoft.com/dotnet/sdk:5.0 as dev

WORKDIR /app

COPY . .

RUN dotnet publish -c Release -r linux-musl-x64 -p:PublishSingleFile=true --self-contained -o ./dist

FROM mcr.microsoft.com/dotnet/runtime-deps:5.0-alpine

WORKDIR /app

COPY --from=dev /app/dist/* ./

ENTRYPOINT ./binary
