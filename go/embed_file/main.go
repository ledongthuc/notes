package main

import (
	"embed"
	_ "embed"
	"fmt"
)

//go:embed "string.txt"
var s string

//go:embed "bytes.txt"
var b []byte

//go:embed "file.txt"
var f embed.FS

func main() {
	fmt.Println("From string.txt: ", s)
	fmt.Println("From bytes.txt: ", string(b))

	data, _ := f.ReadFile("file.txt")
	fmt.Println("From file.txt: ", string(data))
}
