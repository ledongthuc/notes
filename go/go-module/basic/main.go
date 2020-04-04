package main

import (
	"fmt"

	"rsc.io/quote"
	quoteV3 "rsc.io/quote/v3"
)

func main() {
	fmt.Println("Hello world: ", getQuote())
	fmt.Println("Hello world 3: ", getQuoteV3())
}

func getQuote() string {
	return quote.Go()
}

func getQuoteV3() string {
	return quoteV3.Concurrency()
}
