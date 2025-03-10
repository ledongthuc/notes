build:
	GOARCH=wasm GOOS=js go build -o web/app.wasm

run: build
	go run main.go
