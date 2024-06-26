package main

import (
	"context"
	"encoding/json"
	"log"
	"net/http"

	"github.com/samborkent/uuidv7"
)

func PostPassengerSet(w http.ResponseWriter, r *http.Request) {
	log.Println("Request passenger set..")

	var passenger Passenger

	err := json.NewDecoder(r.Body).Decode(&passenger)
	if err != nil {
		message := "Error decoding json!" + err.Error()
		http.Error(w, err.Error(), http.StatusBadRequest) // error 400
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
	uuid := uuidv7.New()

	result, err := db.ExecContext(context.Background(), `INSERT INTO passengers (id, fio, phone, category, sex, description, eks) VALUES
		(?, ?, ?, ?, ?, ?, ?);`, uuid.String(), passenger.Fio, passenger.Phone, passenger.Category, passenger.Sex, passenger.Description, passenger.Eks)
	if err != nil {
		message = "Ошибка добавления пассажира: " + err.Error()
		http.Error(w, message, http.StatusExpectationFailed) // 417
		log.Println(message)
		return
	}
	var id_ int64
	id_, err = result.LastInsertId()
	if err != nil {
		message = "Ошибка добавления пассажира: " + err.Error()
		http.Error(w, message, http.StatusExpectationFailed) // 417
		log.Println(message)
		return
	}
	if id_ > 0 {
		message = "Добавлен пассажир: " + passenger.Fio
		log.Println(message)
	}

	documentResponse := Response{Id: uuid.String(), Message: message}
	response := DocumentResponse{Document_: documentResponse}
	w.Header().Set("Content-Type", cContentTypeJson)
	json.NewEncoder(w).Encode(response)
}
