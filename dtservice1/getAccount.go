package main

import (
	"context"
	"database/sql"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"log"
	"net/http"

	"golang.org/x/crypto/blake2b"
)

func GetAccount(w http.ResponseWriter, r *http.Request) {
	log.Println("Request account data..")

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
	password_ := r.FormValue("pass")
	if len(login_) > 0 && len(password_) > 0 {
		row = db.QueryRowContext(context.Background(), `SELECT * FROM accounts WHERE login=?;`, login_)
	} else {
		message = "Parameters not found!"
		log.Print(message)
		http.Error(w, message, http.StatusBadRequest) // 400
		return
	}

	err = row.Scan(&account.Id, &account.Login, &account.Pass)
	if err != nil {
		if err.Error() == "sql: no rows in result set" {
			message = "Неверный логин или пароль: " + err.Error()
		} else {
			message = "Error get account data: " + err.Error()
		}
		log.Println(message)
		http.Error(w, message, http.StatusExpectationFailed) // 417
		return
	}

	var response Account
	// blake2b ->
	bpassword_ := hex.EncodeToString(NewBlake2b256([]byte(password_ + account.Id)))
	fmt.Println(bpassword_)
	// blake2b <-
	if account.Pass == bpassword_ {
		// acc найден, получить fio & role ->
		row = db.QueryRowContext(context.Background(), `SELECT fio, id_role FROM employees WHERE id=?;`, account.Id)

		err = row.Scan(&account.Fio, &account.Role)
		if err != nil {
			if err.Error() == "sql: no rows in result set" {
				message = "Сотрудник в БД не найден: " + err.Error()
			} else {
				message = "Error get account data: " + err.Error()
			}
			log.Println(message)
			http.Error(w, message, http.StatusExpectationFailed) // 417
			return
		}
		// acc найден, получить fio & role <-
		response = Account{Id: account.Id, Fio: account.Fio, Role: account.Role}
	} else {
		message = "Неверный пароль!"
		log.Println(message)
		http.Error(w, message, http.StatusExpectationFailed) // 417
		return
	}

	w.Header().Set("Content-Type", cContentTypeJson)
	json.NewEncoder(w).Encode(response)
}

func NewBlake2b512(data []byte) []byte {
	hash := blake2b.Sum512(data)
	return hash[:]
}

func NewBlake2b256(data []byte) []byte {
	hash := blake2b.Sum256(data)
	return hash[:]
}
