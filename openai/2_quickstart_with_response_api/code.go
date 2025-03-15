package quickstart_with_response_api

import (
	"fmt"
	"log"
	"openaitest/internal/openairequest"
	"os"
)

func Run() {
	token := os.Getenv("OPEN_AI_KEY")
	if len(token) == 0 {
		panic("Fail to load variable environment OPEN_AI_KEY")
	}

	model := os.Getenv("OPEN_AI_MODEL")
	if len(model) == 0 {
		model = "gpt-4o-mini-2024-07-18"
	}

	resp, err := openairequest.CallResponse(token, model, "Write a one-sentence bedtime story about a unicorn.")
	if err != nil {
		log.Fatalf("Fail to call request: %v", err)
	}

	// Print response
	if len(resp.Output) > 0 {
		fmt.Println("Response:", resp.Output[0].Content[0].Text)
	} else {
		fmt.Println("No response content received")
	}
}
