package main

import (
	"context"
	"encoding/json"
	"log"
	"net/http"

	"github.com/samborkent/uuidv7"
)

func PostEmployeeSet(w http.ResponseWriter, r *http.Request) {
	log.Println("Request employee set..")

	var emp Employee
	uuid := uuidv7.New()
	log.Println(uuid.String())

	err := json.NewDecoder(r.Body).Decode(&emp)
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

	var message, state string
	result, err := db.ExecContext(context.Background(), `INSERT INTO employees (date, timework, id, fio, uchastok, smena, rank, sex, is_busy, phone_work, phone_personal, tab_number, type_work) VALUES
		(?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);`, emp.Date, emp.Timework, uuid.String(), emp.Fio, emp.Uchastok, emp.Smena, emp.Rank, emp.Sex, 0, emp.Phone_work, emp.Phone_personal, emp.Tab_number, emp.Type_work)
	if err != nil {
		message = "Ошибка добавления сотрудника: " + err.Error()
		http.Error(w, message, http.StatusExpectationFailed) // 417
		log.Println(message)
		return
	}

	var id int64
	id, err = result.LastInsertId()
	if err != nil {
		message = "Ошибка добавления сотрудника: " + err.Error()
		http.Error(w, message, http.StatusExpectationFailed) // 417
		log.Println(message)
		return
	}
	if id > 0 {
		message = "Добавлен сотрудник: " + emp.Fio
	}

	log.Printf("Работник %s зарегистрирован: ", emp.Fio)
	documentResponse := Response{State: state, Message: message}
	response := DocumentResponse{Document_: documentResponse}
	w.Header().Set("Content-Type", cContentTypeJson)
	json.NewEncoder(w).Encode(response)
}
