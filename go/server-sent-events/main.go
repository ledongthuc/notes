package main

import (
	"fmt"
	"net/http"
	"time"
)

func sseHandler(w http.ResponseWriter, r *http.Request) {
	// Set headers for SSE
	w.Header().Set("Content-Type", "text/event-stream")
	w.Header().Set("Cache-Control", "no-cache")
	w.Header().Set("Connection", "keep-alive")
	w.Header().Set("Access-Control-Allow-Origin", "*")

	// Check if the ResponseWriter supports flushing
	flusher, ok := w.(http.Flusher)
	if !ok {
		http.Error(w, "Streaming unsupported", http.StatusInternalServerError)
		return
	}

	// Sending events every second
	for i := 0; i < 10; i++ {
		// Write SSE event format
		fmt.Fprintf(w, "event:welcome\ndata: Message %d\n\n", i)

		// Flush the response to send the event immediately
		flusher.Flush()

		// Simulate a delay
		time.Sleep(time.Second)
	}
}

func main() {
	http.HandleFunc("/events", sseHandler)
	fmt.Println("Server listening on :8080")
	http.ListenAndServe(":8080", nil)
}
