package main

import "fmt"

func main() {
	listInts := []int{1, 5, 3, 2, 15}
	for index, v := range listInts {
		fmt.Printf("Test new variable in loop: index: %d - value: %d - address (different address): %v\n", index, v, &v)
	}

	for i := range 10 {
		fmt.Println("Test range number: index: ", i)
	}
}
