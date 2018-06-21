package main

import (
	"net/http"
	"fmt"
)

func main() {
	http.HandleFunc("/", HelloWorldHandler)

	fmt.Println("Listening on localhost:8080")
	http.ListenAndServe(":8080", nil)
}
