# ln ../main.go ./main.go; docker build .
FROM golang:1.10

WORKDIR $GOPATH/src/

COPY ./ ./

RUN ls -la
RUN cat main.go
RUN go build main.go

# Run the executable
CMD ["main"]
