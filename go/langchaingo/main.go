package main

import (
	"context"
	"fmt"

	"github.com/tmc/langchaingo/agents"
	"github.com/tmc/langchaingo/chains"
	"github.com/tmc/langchaingo/llms/openai"
	"github.com/tmc/langchaingo/tools"
)

func main() {
	// search, err := serpapi.New()
	// if err != nil {
	// 	panic(err)
	// }
	agentTools := []tools.Tool{
		tools.Calculator{},
		// search,
	}

	// llm, err := ollama.New(ollama.WithModel("llama3.2"))
	// if err != nil {
	// 	panic(err)
	// }

	// llm, err := ollama.New(ollama.WithModel("llama3.1:8b"))
	// if err != nil {
	// 	panic(err)
	// }

	llm, err := openai.New(
		// openai.WithModel("meta-llama/Llama-3.3-70B-Instruct-Turbo"),
		// openai.WithModel("mistralai/Mistral-Small-24B-Instruct-2501"),
		// openai.WithModel("meta-llama/Llama-4-Scout-17B-16E-Instruct"),

		openai.WithBaseURL("https://api.together.xyz/v1"),
	)
	if err != nil {
		panic(err)
	}

	prompt := "1+1="
	executor, err := agents.Initialize(
		llm,
		agentTools,
		agents.ZeroShotReactDescription,
		agents.WithMaxIterations(3),
	)
	if err != nil {
		panic(err)
	}

	answer, err := chains.Run(context.Background(), executor, prompt)
	fmt.Println(answer)
}
