# backend/infra/hyperlane/relayer/Dockerfile
FROM gcr.io/hyperlane-labs/hyperlane-agent:latest

WORKDIR /app

COPY config.json ./config.json

CMD ["hyperlane", "relayer", "--config", "/app/config.json"]
