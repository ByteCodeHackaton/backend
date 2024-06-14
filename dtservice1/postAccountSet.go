package main

import (
	"context"
	"encoding/hex"
	"encoding/json"
	"log"
	"net/http"
)

func PostAccountSet(w http.ResponseWriter, r *http.Request) {
	log.Println("Request acount set..")

	var account Account

	err := json.NewDecoder(r.Body).Decode(&account)
	if err != nil {
		message := "Error decoding json!" + err.Error()
		http.Error(w, err.Error(), http.StatusBadRequest) // 400
		log.Println(message)
		return
	}

	err = initDatabase(configuration.DbPath)
	if err != nil {
		log.Fatal("error initializing DB connection: ", err)
	}
	err = db.Ping()
	if err != nil {
		log.Fatal("error initializing DB connection: ping error: ", err)
	}
	log.Println("database initialized..")

	var message string

	bpassword_ := hex.EncodeToString(NewBlake2b256([]byte(account.Pass + account.Id)))
	log.Println(bpassword_)

	result, err := db.ExecContext(context.Background(), `INSERT INTO accounts (id_employee, login, password) VALUES (?, ?, ?);`, account.Id, account.Login, bpassword_)
	if err != nil {
		message = "Ошибка добавления аккаунта: " + err.Error()
		http.Error(w, message, http.StatusExpectationFailed) // 417
		log.Println(message)
		return
	}

	var id int64
	id, err = result.LastInsertId()
	if err != nil {
		message = "Ошибка добавления аккаунта: " + err.Error()
		http.Error(w, message, http.StatusExpectationFailed) // 417
		log.Println(message)
		return
	}
	if id > 0 {
		message = "Аккаунт успешно добавлен"
	}

	log.Printf("Аккаунт %s добавлен: ", account.Id)
	response := Account{Id: account.Id}
	w.Header().Set("Content-Type", cContentTypeJson)
	json.NewEncoder(w).Encode(response)
}
