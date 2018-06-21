package main

import (
	"fmt"
	"net/http"

	"github.com/subosito/gotenv"
)

func main() {
	gotenv.Load(".env")

	http.HandleFunc("/webhook", WebHookHandler)
	http.HandleFunc("/kyler", kylerHandler)
	http.HandleFunc("/", HelloWorldHandler)

	fmt.Println("Listening on localhost:8080")
	http.ListenAndServe(":8080", nil)
}
