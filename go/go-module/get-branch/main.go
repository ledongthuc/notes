package main

import (
	"fmt"

	"rsc.io/quote"
)

func main() {
	fmt.Println("Hello world: ", getQuote())
}

func getQuote() string {
	return quote.Hello()
}
