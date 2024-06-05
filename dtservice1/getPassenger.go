package main

import (
	"context"
	"database/sql"
	"encoding/json"
	"log"
	"net/http"
)

func GetPassenger(w http.ResponseWriter, r *http.Request) {
	log.Println("Request passenger ..")

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
	var passenger []Passenger
	var pass Passenger
	var row *sql.Row

	fio_ := r.FormValue("fio")
	phone_ := r.FormValue("phone")
	if len(fio_) > 0 {
		row = db.QueryRowContext(context.Background(), `SELECT * FROM passengers WHERE fio=?;`, fio_)
	} else if len(phone_) > 0 {
		row = db.QueryRowContext(context.Background(), `SELECT * FROM passengers WHERE phone=?;`, phone_)
	} else {
		message = "Parameters not found!"
		http.Error(w, message, http.StatusUnauthorized) // 401
		log.Fatal(message)
		return
	}

	err = row.Scan(&pass.Id, &pass.Fio, &pass.Phone, &pass.Category, &pass.Sex, &pass.Description, &pass.Eks)
	if err != nil {
		if err.Error() == "sql: no rows in result set" {
			message = "Нет такого пассажира в БД: " + err.Error()
		} else {
			message = "Error get passenger: " + err.Error()
		}
		log.Println(message)
		http.Error(w, message, http.StatusExpectationFailed) // 417
		return
	}
	passenger = append(passenger, pass)

	if len(passenger) == 0 {
		message = "Passenger not found!"
		http.Error(w, message, http.StatusNoContent) // 204
		return
	}

	response := Passenger{Id: pass.Id, Fio: pass.Fio, Phone: pass.Phone, Category: pass.Category, Sex: pass.Sex, Description: pass.Description, Eks: pass.Eks}
	w.Header().Set("Content-Type", cContentTypeJson)
	json.NewEncoder(w).Encode(response)
}
