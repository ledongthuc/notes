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
	numberOfSuccess := int64(5)
	if c, err := strconv.ParseInt(os.Getenv("NUMBER_OF_SUCCESS"), 10, 64); err != nil {
		numberOfSuccess = c
	}

	numberOfReadnessFail := int64(5)
	if c, err := strconv.ParseInt(os.Getenv("NUMBER_OF_READNESS_FAIL"), 10, 64); err != nil {
		numberOfReadnessFail = c
	}

	numberOfStartupFail := int64(5)
	if c, err := strconv.ParseInt(os.Getenv("NUMBER_OF_STARTUP_FAIL"), 10, 64); err != nil {
		numberOfStartupFail = c
	}

	http.HandleFunc("/liveness_probe_status", func(w http.ResponseWriter, r *http.Request) {
		if counter >= numberOfSuccess {
			fmt.Println("/liveness_probe_status, Counter: ", counter, ", Status: 500")
			w.WriteHeader(http.StatusInternalServerError)
			return
		}
		counter++
		fmt.Println("/liveness_probe_status, Counter: ", counter, ", Status: 200")
		w.WriteHeader(http.StatusOK)
	})

	http.HandleFunc("/liveness_probe_timeout", func(w http.ResponseWriter, r *http.Request) {
		if counter >= numberOfSuccess {
			fmt.Println("/liveness_probe_timeout, Counter: ", counter, ", Timeout: 1h")
			time.Sleep(1 * time.Hour)
			return
		}
		counter++
		fmt.Println("/liveness_probe_timeout, Counter: ", counter, ", Status: Success")
		w.WriteHeader(http.StatusOK)
	})

	http.HandleFunc("/readness_probe_status", func(w http.ResponseWriter, r *http.Request) {
		if counter >= numberOfReadnessFail {
			fmt.Println("/readness_probe_status, Counter: ", counter, ", Status: 500")
			w.WriteHeader(http.StatusInternalServerError)
			return
		}
		counter++
		fmt.Println("/readness_probe_status, Counter: ", counter, ", Status: 200")
		w.WriteHeader(http.StatusOK)
	})

	http.HandleFunc("/readness_probe_timeout", func(w http.ResponseWriter, r *http.Request) {
		if counter >= numberOfReadnessFail {
			fmt.Println("/readness_probe_timeout, Counter: ", counter, ", Timeout: 1h")
			time.Sleep(1 * time.Hour)
			return
		}
		counter++
		fmt.Println("/readness_probe_timeout, Counter: ", counter, ", Status: 200")
		w.WriteHeader(http.StatusOK)
	})

	http.HandleFunc("/startup_probe_status", func(w http.ResponseWriter, r *http.Request) {
		if counter >= numberOfStartupFail {
			fmt.Println("/startup_probe_status, Counter: ", counter, ", Status: 500")
			w.WriteHeader(http.StatusInternalServerError)
			return
		}
		counter++
		fmt.Println("/startup_probe_status, Counter: ", counter, ", Status: 200")
		w.WriteHeader(http.StatusOK)
	})

	http.HandleFunc("/startup_probe_timeout", func(w http.ResponseWriter, r *http.Request) {
		if counter >= numberOfStartupFail {
			fmt.Println("/startup_probe_timeout, Counter: ", counter, ", Timeout: 1h")
			time.Sleep(1 * time.Hour)
			return
		}
		counter++
		fmt.Println("/startup_probe_timeout, Counter: ", counter, ", Status: 200")
		w.WriteHeader(http.StatusOK)
	})

	fmt.Println("Start with port 8080")
	log.Fatal(http.ListenAndServe(":8080", nil))
}
