package main

import (
	"fmt"
	"net/http"
	"database/sql"
)

type User struct {
	id int
	username string
	slack_id string
}

func WebHookHandler(w http.ResponseWriter, r *http.Request, dbinfo string) {
	user := new(User)

	r.ParseForm()
	if len(r.Form["user_name"]) != 1 {
		w.Write([]byte(fmt.Sprintf(`{"text":"No username specified"}`)))
		return
	}
	user.username = r.Form["user_name"][0]

	db, _ := sql.Open("postgres", dbinfo)
  defer db.Close()

	err := db.QueryRow("SELECT id, slack_id FROM users WHERE username=$1", user.username).Scan(&user.id, &user.slack_id)

	if err == nil {
		fmt.Printf("found user: %d - %s\n", user.id, user.slack_id)
	} else if err == sql.ErrNoRows {
		w.Write([]byte(fmt.Sprintf(`{"text":"Could not find user %s"}`, user.username)))
		return
	} else {
		fmt.Printf("%s", err)
		return
	}

	w.Write([]byte(fmt.Sprintf(`{"text":"Hello, %s"}`, user.slack_id)))
}
