package main

import (
	"fmt"
	"log"
	"net/http"
	"os"
	"strconv"
	"time"
)

func main() {
	var counterLivenessSuccess = int64(0)
	var counterStartupSkip = int64(0)

	fmt.Println("Env NUMBER_OF_LIVENESS_SUCCESS: ", os.Getenv("NUMBER_OF_LIVENESS_SUCCESS"))
	fmt.Println("Env NUMBER_OF_SKIP_STARTUP: ", os.Getenv("NUMBER_OF_SKIP_STARTUP"))

	numberOfStartupSkip := int64(3)
	if c, err := strconv.ParseInt(os.Getenv("NUMBER_OF_LIVENESS_SUCCESS"), 10, 64); err == nil {
		numberOfStartupSkip = c
	}

	http.HandleFunc("/liveness_probe_startup_checking", func(w http.ResponseWriter, r *http.Request) {
		fmt.Println("/liveness_probe_startup_checking, Counter: ", counterStartupSkip)
		if counterStartupSkip >= numberOfStartupSkip {
			fmt.Println("/liveness_probe_startup_checking, Ready")
			w.WriteHeader(http.StatusOK)
			return
		}

		w.WriteHeader(http.StatusInternalServerError)
	})

	numberOfSuccess := int64(5)
	if c, err := strconv.ParseInt(os.Getenv("NUMBER_OF_LIVENESS_SUCCESS"), 10, 64); err == nil {
		numberOfSuccess = c
	}

	http.HandleFunc("/liveness_probe_status", func(w http.ResponseWriter, r *http.Request) {
		fmt.Println("/liveness_probe_status, Counter: ", counterLivenessSuccess)
		if counterLivenessSuccess >= numberOfSuccess {
			fmt.Println("/liveness_probe_status, Error result")
			w.WriteHeader(http.StatusInternalServerError)
			return
		}
		counterLivenessSuccess++
		w.WriteHeader(http.StatusOK)
	})

	http.HandleFunc("/liveness_probe_timeout", func(w http.ResponseWriter, r *http.Request) {
		fmt.Println("/liveness_probe_timeout, Counter: ", counterLivenessSuccess)
		if counterLivenessSuccess >= numberOfSuccess {
			fmt.Println("/liveness_probe_timeout, Timeout")
			time.Sleep(1 * time.Hour)
			return
		}
		counterLivenessSuccess++
		w.WriteHeader(http.StatusOK)
	})

	fmt.Println("Start with port 8080")
	log.Fatal(http.ListenAndServe(":8080", nil))
}
