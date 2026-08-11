docker run -d \
  --name mongodb \
  -e MONGO_INITDB_ROOT_USERNAME=admin \
  -e MONGO_INITDB_ROOT_PASSWORD=123 \
  -p 27017:27017 \
  mongo:latest

MONGODB_URI="mongodb://admin:123@localhost:27017/?retryWrites=true&w=majority" go run main.go
MONGODB_URI="mongodb://admin:123@localhost:27017/?retryWrites=true&w=majority" go test ./...
