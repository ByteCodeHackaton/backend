package main

import (
	"context"
	"encoding/json"
	"log"
	"net/http"
)

func PostPassengerUpdate(w http.ResponseWriter, r *http.Request) {
	log.Println("Request passenger update..")

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
	//uuid_ := r.FormValue("id")

	result, err := db.ExecContext(context.Background(), `UPDATE passengers SET fio=?, phone=?, category=?, sex=?, description=?, eks=? WHERE id=?;`,
		passenger.Fio, passenger.Phone, passenger.Category, passenger.Sex, passenger.Description, passenger.Eks, passenger.Id)

	if err != nil {
		message = "Ошибка изменения информации о пассажире: " + err.Error()
		http.Error(w, message, http.StatusExpectationFailed) // 417
		log.Println(message)
		return
	}
	var id_ int64
	id_, err = result.RowsAffected()
	if err != nil {
		message = "Ошибка изменения информации о пассажире: " + err.Error()
		http.Error(w, message, http.StatusExpectationFailed) // 417
		log.Println(message)
		return
	}
	if id_ > 0 {
		message = "Изменена информация о пассажире: " + passenger.Fio
		log.Println(message)
	}

	response := Passenger{Id: passenger.Id}
	w.Header().Set("Content-Type", cContentTypeJson)
	json.NewEncoder(w).Encode(response)
}
