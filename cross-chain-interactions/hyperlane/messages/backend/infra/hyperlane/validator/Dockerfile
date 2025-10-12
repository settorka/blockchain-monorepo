
FROM gcr.io/hyperlane-labs/hyperlane-agent:latest

WORKDIR /app

COPY config.json ./config.json

CMD ["hyperlane", "validator", "--config", "/app/config.json"]
