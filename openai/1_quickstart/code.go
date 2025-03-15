package quickstart

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

	resp, err := openairequest.CallCompletion(token, model, []openairequest.Message{
		{
			Role:    "user",
			Content: "Write a one-sentence bedtime story about a unicorn.",
		},
	})
	if err != nil {
		log.Fatalf("Fail to call request: %v", err)
	}

	// Print response
	if len(resp.Choices) > 0 {
		fmt.Println("Response:", resp.Choices[0].Message.Content)
	} else {
		fmt.Println("No response content received")
	}
}
