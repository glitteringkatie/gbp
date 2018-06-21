package main

import (
	"net/http"
	"fmt"
)

func main() {
	http.HandleFunc("/", HelloWorldHandler)
	http.HandleFunc("/kyler", kylerHandler)
	fmt.Println("Listening on localhost:8080")
	http.ListenAndServe(":8080", nil)
}
