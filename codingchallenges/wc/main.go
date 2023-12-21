package main

import (
	"bufio"
	"flag"
	"fmt"
	"io"
	"os"
	"unicode"
)

var flagvar int

func init() {
}

func main() {
	isBytesCount := flag.Bool("c", false, "bytes count")
	isLinesCount := flag.Bool("l", false, "lines count")
	isWordCount := flag.Bool("w", false, "word count")
	isCharacterCount := flag.Bool("m", false, "character count")
	flag.Parse()

	var reader *bufio.Reader
	if len(flag.Args()) >= 1 {
		f, err := os.Open(flag.Args()[len(flag.Args())-1])
		if err != nil {
			panic(err)
		}
		reader = bufio.NewReader(f)
	} else {
		reader = bufio.NewReader(os.Stdin)
	}

	var bytesCount, linesCount, wordsCount, charactersCount int64
	var isInWord = true

	for {

		r, size, err := reader.ReadRune()
		if err != nil {
			if err == io.EOF {
				if isInWord {
					wordsCount++
				}
				break
			}
			panic(err)
		}

		bytesCount += int64(size)
		if unicode.IsSpace(r) {
			if isInWord {
				wordsCount += 1
				isInWord = false
			}
			if r == '\n' {
				linesCount += 1
			}
		} else {
			isInWord = true
		}
		charactersCount += 1
	}

	switch {
	case isBytesCount != nil && *isBytesCount:
		fmt.Println(bytesCount)
	case isLinesCount != nil && *isLinesCount:
		fmt.Println(linesCount)
	case isWordCount != nil && *isWordCount:
		fmt.Println(wordsCount)
	case isCharacterCount != nil && *isCharacterCount:
		fmt.Println(charactersCount)
	default:
		fmt.Printf("%d\t%d\t%d\n", linesCount, wordsCount, bytesCount)
	}
}
