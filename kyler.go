package main

import "net/http"

func kylerHandler(w http.ResponseWriter, r *http.Request) {
	w.Write([]byte("Hello Kyler"))
}
