FROM golang:latest as builder
WORKDIR /app
COPY . .
RUN go get; CGO_ENABLED=0 GOOS=linux go build -a -installsuffix cgo -o app .

FROM alpine:latest  
WORKDIR /root/
COPY --from=builder /app/app .
CMD ["./app"] 
