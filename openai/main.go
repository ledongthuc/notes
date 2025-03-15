package main

import (
	"fmt"

	quickstart "openaitest/1_quickstart"
	quickstart_with_response_api "openaitest/2_quickstart_with_response_api"
)

type example struct {
	Name string
	Func func()
}

func main() {
	examples := map[string]example{
		"1": {Name: "Quickstart", Func: quickstart.Run},
		"2": {Name: "Quickstart with Response API", Func: quickstart_with_response_api.Run},
	}
	for {
		fmt.Println("\nAvailable examples:")
		fmt.Println("------------------")
		for key, example := range examples {
			fmt.Printf("%s: %s\n", key, example.Name)
		}
		fmt.Println("q: Quit")
		fmt.Println("\nPlease choose an example to run (enter the number or 'q' to quit):")

		var choice string
		fmt.Scanln(&choice)

		if choice == "q" {
			fmt.Println("Goodbye!")
			return
		}

		if example, exists := examples[choice]; exists {
			example.Func()
		} else {
			fmt.Println("Invalid choice. Please try again.")
		}
	}
}
