package main

import (
	"fmt"
	"log"
	"net/http"
)

const (
	CookieAuthenticate = "AUTHENTICATED"
)

func main() {
	http.HandleFunc("/login", func(w http.ResponseWriter, r *http.Request) {
		http.SetCookie(w, &http.Cookie{
			Name:  CookieAuthenticate,
			Value: "true",
		})
		w.WriteHeader(http.StatusOK)
	})

	http.HandleFunc("/logout", func(w http.ResponseWriter, r *http.Request) {
		http.SetCookie(w, &http.Cookie{
			Name:   CookieAuthenticate,
			Value:  "false",
			MaxAge: 0,
		})
		w.WriteHeader(http.StatusOK)
	})

	http.HandleFunc("/authenticate", func(w http.ResponseWriter, r *http.Request) {
		c, err := r.Cookie(CookieAuthenticate)
		if err != nil {
			fmt.Fprintf(w, err.Error())
			w.WriteHeader(http.StatusForbidden)
			return
		}

		if c == nil || c.Value == "true" {
			fmt.Fprintf(w, "Forbidden")
			w.WriteHeader(http.StatusForbidden)
			return
		}

		w.WriteHeader(http.StatusOK)
	})

	log.Fatal(http.ListenAndServe("localhost:8080", nil))
}
