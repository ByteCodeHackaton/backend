package main

import (
	"context"
	"encoding/json"
	"log"
	"net/http"
)

func GetPassengerList(w http.ResponseWriter, r *http.Request) {
	log.Println("Request passenger list..")

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
	rows, err := db.QueryContext(context.Background(), `SELECT * FROM passengers`)

	var message, state string
	// var is_busy int
	// var cur_station NullInt64 // по умолчанию null

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

	documentResponse := ResponsePassenger{State: state, Message: message, Passenger: passenger}
	response := DocumentResponsePassenger{Document_: documentResponse}
	w.Header().Set("Content-Type", cContentTypeJson)
	json.NewEncoder(w).Encode(response)
}
