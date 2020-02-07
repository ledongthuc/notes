package main

import (
	"fmt"
	"log"
	"net/http"
	"os"
)

const (
	CookieAuthenticate = "AUTHENTICATED"
	EnvUsername        = "USERNAME"
	EnvPassword        = "PASSWORD"
)

func main() {
	username := "username"
	if os.Getenv(EnvUsername) != "" {
		username = os.Getenv(EnvUsername)
	}
	password := "password"
	if os.Getenv(EnvPassword) != "" {
		password = os.Getenv(EnvPassword)
	}
	fmt.Printf("%s: %s\n%s: %s\n", EnvUsername, username, EnvPassword, password)

	http.HandleFunc("/login", func(w http.ResponseWriter, r *http.Request) {
		usernameP := r.URL.Query()["username"]
		passwordP := r.URL.Query()["password"]
		if len(usernameP) == 0 || usernameP[0] != username ||
			len(passwordP) == 0 || passwordP[0] != password {
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
