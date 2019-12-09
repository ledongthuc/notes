package main

import (
	"fmt"
	"log"
	"net/http"
	"os"
	"strconv"
	"time"
)

var counter = int64(0)

func main() {
	fmt.Println("Env NUMBER_OF_SUCCESS: ", os.Getenv("NUMBER_OF_SUCCESS"))

	numberOfSuccess := int64(5)
	if c, err := strconv.ParseInt(os.Getenv("NUMBER_OF_SUCCESS"), 10, 64); err == nil {
		numberOfSuccess = c
	}

	http.HandleFunc("/liveness_probe_status", func(w http.ResponseWriter, r *http.Request) {
		fmt.Println("/liveness_probe_status, Counter: ", counter)
		if counter >= numberOfSuccess {
			w.WriteHeader(http.StatusInternalServerError)
			return
		}
		counter++
		w.WriteHeader(http.StatusOK)
	})

	http.HandleFunc("/liveness_probe_timeout", func(w http.ResponseWriter, r *http.Request) {
		fmt.Println("/liveness_probe_timeout, Counter: ", counter)
		if counter >= numberOfSuccess {
			time.Sleep(1 * time.Hour)
			return
		}
		counter++
		w.WriteHeader(http.StatusOK)
	})

	fmt.Println("Start with port 8080")
	log.Fatal(http.ListenAndServe(":8080", nil))
}
