package main

import (
	"fmt"
	"net/http"
)

func hello(w http.ResponseWriter, req *http.Request) {
	fmt.Fprintf(w, "hello\n")
}

func helloName(w http.ResponseWriter, req *http.Request) {
	fmt.Fprintf(w, "hello '%s'\n", req.PathValue("name"))
}

func main() {
	http.HandleFunc("GET /hello", hello)
	http.HandleFunc("GET /hello/{name}", helloName)
	http.ListenAndServe(":8090", nil)
}
