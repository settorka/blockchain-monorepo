FROM golang:1.22-alpine

WORKDIR /app
COPY ../../ ./

RUN go mod tidy && go build -o txpool-builder ./cmd/builder

ENV GETH_RPC_URL=http://geth:8545

CMD ["./txpool-builder"]
