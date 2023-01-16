Generate json schema from proto file.

1. Install protoc:

https://developers.google.com/protocol-buffers/docs/downloads

2. Install

```
go install google.golang.org/protobuf/cmd/protoc-gen-go@latest
go install github.com/favadi/protoc-go-inject-tag@latest
```

3. Generate proto files
protoc -I=./proto_files --go_out=./  ./proto_files/student.proto ./proto_files/class.proto

4. Inject json schema to proto files
protoc-go-inject-tag -input="./app/pro/*.pb.go"

5. Generate json schema
```go
jsonSchemaBytes, err := jsonschema.Reflect(&pro.Student{}).MarshalJSON()
```
