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
		user := r.URL.Query()["user"]
		if len(user) == 0 || user[0] != "password" {
			w.WriteHeader(http.StatusUnauthorized)
			w.Write([]byte("Login fail"))
			return
		}
		http.SetCookie(w, &http.Cookie{
			Name:  CookieAuthenticate,
			Value: "true",
		})
		w.WriteHeader(http.StatusOK)
		w.Write([]byte("Login success"))
	})

	http.HandleFunc("/logout", func(w http.ResponseWriter, r *http.Request) {
		http.SetCookie(w, &http.Cookie{
			Name:   CookieAuthenticate,
			Value:  "false",
			MaxAge: 0,
		})
		w.WriteHeader(http.StatusOK)
		w.Write([]byte("Logout success"))
	})

	http.HandleFunc("/authenticate", func(w http.ResponseWriter, r *http.Request) {
		c, err := r.Cookie(CookieAuthenticate)
		if err != nil {
			w.WriteHeader(http.StatusUnauthorized)
			fmt.Fprintf(w, err.Error())
			return
		}

		if c == nil || c.Value == "false" {
			w.WriteHeader(http.StatusUnauthorized)
			fmt.Fprintf(w, "Forbidden")
			return
		}

		w.WriteHeader(http.StatusOK)
		w.Write([]byte("Authenticated"))
	})

	fmt.Println("Start server")
	log.Fatal(http.ListenAndServe(":8080", nil))
}
