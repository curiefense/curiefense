FROM golang:1-alpine as builder

WORKDIR /app
COPY curielogger .
RUN go build -o build/curielogger ./cmd/grpc

FROM alpine
RUN apk update && apk add ca-certificates curl bash && rm -rf /var/cache/apk/*
RUN mkdir -p /etc/curielogger/
COPY contrib /etc/curielogger/contrib
COPY --from=builder /app/build/curielogger /bin
COPY --from=builder /app/curielogger.yml /etc/curielogger/
ENTRYPOINT ["/bin/curielogger"]
