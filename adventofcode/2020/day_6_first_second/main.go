package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

func main() {
	content, err := ParseInput("./declare.txt")
	if err != nil {
		panic(err)
	}

	groups := strings.Split(strings.TrimSpace(content), "\n\n")
	var count int
	for _, group := range groups {
		count += countAllYes(group)
	}

	fmt.Printf("Total group %d: %d\n", len(groups), count)
}

func countAllYes(group string) int {
	noPeople := strings.Count(group, "\n") + 1
	m := map[rune]int{}
	var count int
	for _, answers := range strings.Split(group, "\n") {
		for _, answer := range answers {
			m[answer] = m[answer] + 1
			if m[answer] == noPeople {
				count++
			}
		}
	}
	return count
}

func ParseInput(filePath string) (string, error) {
	content, err := ioutil.ReadFile(filePath)
	if err != nil {
		return "", err
	}
	return string(content), nil
}
