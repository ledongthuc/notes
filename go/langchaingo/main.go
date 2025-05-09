package main

import (
	"context"
	"fmt"

	"github.com/tmc/langchaingo/agents"
	"github.com/tmc/langchaingo/chains"
	"github.com/tmc/langchaingo/llms/ollama"
	"github.com/tmc/langchaingo/tools"
	"github.com/tmc/langchaingo/tools/serpapi"
)

func main() {
	search, err := serpapi.New()
	if err != nil {
		panic(err)
	}
	agentTools := []tools.Tool{
		tools.Calculator{},
		search,
	}

	llm, err := ollama.New(ollama.WithModel("llama3.2"))
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
	panic(err)
}
