package main

import (
	"context"
	"database/sql"
	"encoding/hex"
	"encoding/json"
	"log"
	"net/http"
)

func GetAccountExist(w http.ResponseWriter, r *http.Request) {
	log.Println("Request account login data..")

	err := initDatabase(configuration.DbPath)
	if err != nil {
		log.Fatal("error initializing DB connection: ", err)
	}
	err = db.Ping()
	if err != nil {
		log.Fatal("error initializing DB connection: ping error: ", err)
	}
	log.Println("database initialized..")

	var message string
	var account Account
	var row *sql.Row

	login_ := r.FormValue("login")
	if len(login_) > 0 {
		blogin_ := hex.EncodeToString(NewBlake2b256([]byte(login_)))
		row = db.QueryRowContext(context.Background(), `SELECT * FROM accounts WHERE login=?;`, blogin_)
	} else {
		message = "Parameter not found!"
		log.Print(message)
		http.Error(w, message, http.StatusBadRequest) // 400
		return
	}

	err = row.Scan(&account.Id, &account.Login, &account.Pass)
	if err != nil {
		if err.Error() == "sql: no rows in result set" {
			message = "Указанный логин не найден в БД: " + err.Error()
		} else {
			message = "Error get account login data: " + err.Error()
		}
		log.Println(message)
		http.Error(w, message, http.StatusExpectationFailed) // 417
		return
	}

	response := Account{Id: account.Id}
	w.Header().Set("Content-Type", cContentTypeJson)
	json.NewEncoder(w).Encode(response)
}
