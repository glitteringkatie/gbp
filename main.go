package main

import (
	"fmt"
	_ "github.com/lib/pq"
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

	dbinfo := fmt.Sprintf("user=%s password=%s dbname=%s sslmode=disable",
            os.Getenv("DB_USER"), os.Getenv("DB_PASS"), os.Getenv("DB_NAME"))

	http.HandleFunc("/webhook", func(w http.ResponseWriter, r *http.Request) {
		WebHookHandler(w, r, dbinfo)

	})
	http.HandleFunc("/kyler", kylerHandler)
	http.HandleFunc("/", HelloWorldHandler)

	fmt.Printf("Listening on :%s\n", port)
	http.ListenAndServe(":"+port, nil)
}
