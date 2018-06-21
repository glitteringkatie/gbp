package main

import (
	"fmt"
	"net/http"
)

func WebHookHandler(w http.ResponseWriter, r *http.Request) {
	r.ParseForm()

	name := r.Form["user_name"]

	w.Write([]byte(fmt.Sprintf(`{"text":"Hello, %s"}`, name)))
}
