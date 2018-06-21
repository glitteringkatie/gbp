package main

import (
	"net/http"
	"fmt"
	"log"
	"os"
	"github.com/subosito/gotenv"
)

func main() {
	gotenv.Load(".env")

	http.HandleFunc("/", HelloWorldHandler)
	http.HandleFunc("/kyler", kylerHandler)
	fmt.Println("Listening on localhost:8080")
	http.ListenAndServe(":8080", nil)
}
