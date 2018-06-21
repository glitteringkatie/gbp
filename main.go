package main

import (
	"fmt"
	"net/http"
)

func main() {
	http.HandleFunc("/webhook", WebHookHandler)
	http.HandleFunc("/kyler", kylerHandler)

	fmt.Println("Listening on localhost:8080")
	http.ListenAndServe(":8080", nil)
}
