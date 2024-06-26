package main

import (
	"context"
	"database/sql"
	"encoding/json"
	"log"
	"net/http"
	"strconv"
)

func GetPassengerList(w http.ResponseWriter, r *http.Request) {
	log.Println("Request passenger list..")

	limit_ := r.FormValue("limit")
	offset_ := r.FormValue("off")

	if len(limit_) == 0 {
		limit_ = "20"
	}

	if len(offset_) == 0 {
		offset_ = "0"
	}

	err := initDatabase(configuration.DbPath)
	if err != nil {
		log.Fatal("error initializing DB connection: ", err)
	}
	err = db.Ping()
	if err != nil {
		log.Fatal("error initializing DB connection: ping error: ", err)
	}
	log.Println("database initialized..")

	var passenger []Passenger
	var message string
	var row *sql.Row
	var total_count, page_count int

	row = db.QueryRowContext(context.Background(), `SELECT Count(*) FROM passengers;`)
	err = row.Scan(&total_count)
	if err != nil {
		message = "Error get count total passengers: " + err.Error()
		log.Println(message)
		http.Error(w, message, http.StatusExpectationFailed) // 417
		return
	}
	limit_i, _ := strconv.Atoi(limit_)
	page_count = total_count/limit_i + 1

	rows, err := db.QueryContext(context.Background(), `SELECT * FROM passengers LIMIT ? OFFSET ?`, limit_, offset_)

	switch err {
	case nil:
		for rows.Next() {
			var pass Passenger

			if err := rows.Scan(&pass.Id, &pass.Fio, &pass.Phone, &pass.Category, &pass.Sex, &pass.Description, &pass.Eks); err != nil {
				message = "Error get passengers list: " + err.Error()
				log.Println(message)
				http.Error(w, message, http.StatusExpectationFailed) // 417
				return
			}
			passenger = append(passenger, pass)
		}
		log.Println("Get passengers list successfull!")
	default:
		message = "Error get passengers list: " + err.Error()
		log.Println(message)
		http.Error(w, message, http.StatusExpectationFailed) // 417
		return
	}
	defer rows.Close()

	if len(passenger) == 0 {
		message = "Passengers list is empty!"
		http.Error(w, message, http.StatusNoContent) // 204
		return
	}

	documentResponse := ResponsePassenger{Total_count: total_count, Page_count: page_count, Passenger: passenger}
	response := DocumentResponsePassenger{Document_: documentResponse}
	w.Header().Set("Content-Type", cContentTypeJson)
	json.NewEncoder(w).Encode(response)
}
