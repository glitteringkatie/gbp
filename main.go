package main

import (
	"fmt"
	"net/http"
	"os"

	"github.com/subosito/gotenv"
)

func main() {
	gotenv.Load(".env")

	port := os.Getenv("PORT")
	if port == "" {
		port = "8080"
	}

	http.HandleFunc("/webhook", WebHookHandler)
	http.HandleFunc("/kyler", kylerHandler)
	http.HandleFunc("/", HelloWorldHandler)

	fmt.Printf("Listening on :%s\n", port)
	http.ListenAndServe(":"+port, nil)
}
