package openairequest

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
)

func CallCompletion(token string, model string, messages []Message) (ChatCompletionResponse, error) {
	// API endpoint
	url := "https://api.openai.com/v1/chat/completions"

	// Create request body
	reqBody := ChatCompletionRequest{
		Model:    model,
		Messages: messages,
	}

	// Convert request body to JSON
	jsonData, err := json.Marshal(reqBody)
	if err != nil {
		return ChatCompletionResponse{}, fmt.Errorf("Error marshaling request: %w", err)
	}

	// Create new HTTP request
	req, err := http.NewRequest("POST", url, bytes.NewBuffer(jsonData))
	if err != nil {
		return ChatCompletionResponse{}, fmt.Errorf("Error creating request: %w", err)
	}

	// Add headers
	req.Header.Set("Content-Type", "application/json")
	req.Header.Set("Authorization", "Bearer "+token)

	// Send request
	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		return ChatCompletionResponse{}, fmt.Errorf("Error sending request: %w", err)
	}
	defer resp.Body.Close()

	// Read response
	body, err := io.ReadAll(resp.Body)
	if err != nil {
		return ChatCompletionResponse{}, fmt.Errorf("Error reading response: %w", err)
	}

	// Parse response
	var response ChatCompletionResponse
	err = json.Unmarshal(body, &response)
	if err != nil {
		return ChatCompletionResponse{}, fmt.Errorf("Error parsing response: %w", err)
	}

	return response, nil
}

func CallResponse(token string, model string, input string) (ResponseResponse, error) {
	// API endpoint
	url := "https://api.openai.com/v1/responses"

	// Create request body
	reqBody := ResponseRequest{
		Model: model,
		Input: input,
	}

	// Convert request body to JSON
	jsonData, err := json.Marshal(reqBody)
	if err != nil {
		return ResponseResponse{}, fmt.Errorf("Error marshaling request: %w", err)
	}

	// Create new HTTP request
	req, err := http.NewRequest("POST", url, bytes.NewBuffer(jsonData))
	if err != nil {
		return ResponseResponse{}, fmt.Errorf("Error creating request: %w", err)
	}

	// Add headers
	req.Header.Set("Content-Type", "application/json")
	req.Header.Set("Authorization", "Bearer "+token)

	// Send request
	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		return ResponseResponse{}, fmt.Errorf("Error sending request: %w", err)
	}
	defer resp.Body.Close()

	// Read response
	body, err := io.ReadAll(resp.Body)
	if err != nil {
		return ResponseResponse{}, fmt.Errorf("Error reading response: %w", err)
	}

	// Parse response
	var response ResponseResponse
	err = json.Unmarshal(body, &response)
	if err != nil {
		return ResponseResponse{}, fmt.Errorf("Error parsing response: %w", err)
	}

	return response, nil
}
