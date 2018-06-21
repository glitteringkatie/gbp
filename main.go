package main

import (
	"net/http"

	"github.com/ngaut/log"
)

func main() {
	http.HandleFunc("/", HelloWorldHandler)

	log.Info("Listening on localhost:8080")
	http.ListenAndServe(":8080", nil)
}
