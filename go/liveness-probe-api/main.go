package main

import (
	"fmt"
	"log"
	"net/http"
	"os"
	"strconv"
	"time"
)

var counterLiveness = int64(0)
var counterReadiness = int64(0)
var counterStartup = int64(0)

func main() {
	numberOfSuccess := int64(5)
	if c, err := strconv.ParseInt(os.Getenv("NUMBER_OF_SUCCESS"), 10, 64); err != nil {
		numberOfSuccess = c
	}

	numberOfReadinessFail := int64(5)
	if c, err := strconv.ParseInt(os.Getenv("NUMBER_OF_READINESS_FAIL"), 10, 64); err != nil {
		numberOfReadinessFail = c
	}

	numberOfStartupFail := int64(5)
	if c, err := strconv.ParseInt(os.Getenv("NUMBER_OF_STARTUP_FAIL"), 10, 64); err != nil {
		numberOfStartupFail = c
	}

	http.HandleFunc("/liveness_probe_status", func(w http.ResponseWriter, r *http.Request) {
		defer func() { counterLiveness++ }()
		if counterLiveness >= numberOfSuccess {
			fmt.Println("/liveness_probe_status, Counter: ", counterLiveness, ", Status: 500")
			w.WriteHeader(http.StatusInternalServerError)
			return
		}
		fmt.Println("/liveness_probe_status, Counter: ", counterLiveness, ", Status: 200")
		w.WriteHeader(http.StatusOK)
	})

	http.HandleFunc("/liveness_probe_timeout", func(w http.ResponseWriter, r *http.Request) {
		defer func() { counterLiveness++ }()
		if counterLiveness >= numberOfSuccess {
			fmt.Println("/liveness_probe_timeout, Counter: ", counterLiveness, ", Timeout: 1h")
			time.Sleep(1 * time.Hour)
			return
		}
		fmt.Println("/liveness_probe_timeout, Counter: ", counterLiveness, ", Status: Success")
		w.WriteHeader(http.StatusOK)
	})

	http.HandleFunc("/readiness_probe_status", func(w http.ResponseWriter, r *http.Request) {
		defer func() { counterReadiness++ }()
		if counterReadiness < numberOfReadinessFail {
			fmt.Println("/readiness_probe_status, Counter: ", counterReadiness, ", Status: 500")
			w.WriteHeader(http.StatusInternalServerError)
			return
		}
		fmt.Println("/readiness_probe_status, Counter: ", counterReadiness, ", Status: 200")
		w.WriteHeader(http.StatusOK)
	})

	http.HandleFunc("/readiness_probe_timeout", func(w http.ResponseWriter, r *http.Request) {
		defer func() { counterReadiness++ }()
		if counterReadiness < numberOfReadinessFail {
			fmt.Println("/readiness_probe_timeout, Counter: ", counterReadiness, ", Timeout: 1h")
			time.Sleep(1 * time.Hour)
			return
		}
		fmt.Println("/readiness_probe_timeout, Counter: ", counterReadiness, ", Status: 200")
		w.WriteHeader(http.StatusOK)
	})

	http.HandleFunc("/startup_probe_status", func(w http.ResponseWriter, r *http.Request) {
		defer func() { counterStartup++ }()
		if counterStartup < numberOfStartupFail {
			fmt.Println("/startup_probe_status, Counter: ", counterStartup, ", Status: 500")
			w.WriteHeader(http.StatusInternalServerError)
			return
		}
		fmt.Println("/startup_probe_status, Counter: ", counterStartup, ", Status: 200")
		w.WriteHeader(http.StatusOK)
	})

	http.HandleFunc("/startup_probe_timeout", func(w http.ResponseWriter, r *http.Request) {
		defer func() { counterStartup++ }()
		if counterStartup < numberOfStartupFail {
			fmt.Println("/startup_probe_timeout, Counter: ", counterStartup, ", Timeout: 1h")
			time.Sleep(1 * time.Hour)
			return
		}
		fmt.Println("/startup_probe_timeout, Counter: ", counterStartup, ", Status: 200")
		w.WriteHeader(http.StatusOK)
	})

	fmt.Println("Start with port 8080")
	log.Fatal(http.ListenAndServe(":8080", nil))
}
