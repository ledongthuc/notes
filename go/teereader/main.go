package main

import (
	"bytes"
	"fmt"
	"io"
)

func main() {
	reader := bytes.NewBufferString("This is content from buffer")

	var newReader1 bytes.Buffer
	newReader2 := io.TeeReader(reader, &newReader1)

	readAReader(newReader2)
	readAReader(&newReader1)
}

func readAReader(r io.Reader) {
	b, err := io.ReadAll(r)
	if err != nil {
		panic(err)
	}
	fmt.Println("Read from reader: ", string(b))
}
