# backend/infra/anvil/Dockerfile
FROM ghcr.io/foundry-rs/foundry:latest

WORKDIR /app

COPY entrypoint.sh /app/entrypoint.sh
RUN chmod +x /app/entrypoint.sh

ENTRYPOINT ["/app/entrypoint.sh"]
