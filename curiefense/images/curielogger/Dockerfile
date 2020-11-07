FROM golang as builder

COPY curielogger /curielogger
WORKDIR /curielogger
RUN go build -o build/curielogger ./curielogger/ 

FROM golang
COPY --from=builder /curielogger/build/curielogger /bin
CMD /bin/curielogger
