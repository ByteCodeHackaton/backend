package main

import (
	"context"
	"encoding/json"
	"log"
	"net/http"

	"github.com/samborkent/uuidv7"
)

func PostWorkdaySet(w http.ResponseWriter, r *http.Request) {
	log.Println("Request workday set..")

	var workday Workday
	uuid := uuidv7.New()
	log.Println(uuid.String())

	err := json.NewDecoder(r.Body).Decode(&workday)
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
	result, err := db.ExecContext(context.Background(), `INSERT INTO workdays (id, employee_id, date_work, time_work, state_wd, date_dop_smena, date_ucheba,
		date_change, intern) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?);`, uuid.String(), workday.Employee_Id, workday.Date_work, workday.Time_work, workday.State_wd,
		workday.Date_dop_smena, workday.Date_ucheba, workday.Date_change, workday.Intern)
	if err != nil {
		message = "Ошибка регистрация рабочего дня сотрудника: " + err.Error()
		http.Error(w, message, http.StatusExpectationFailed) // 417
		log.Println(message)
		return
	}

	var id int64
	id, err = result.LastInsertId()
	if err != nil {
		message = "Ошибка регистрация рабочего дня сотрудника: " + err.Error()
		http.Error(w, message, http.StatusExpectationFailed) // 417
		log.Println(message)
		return
	}
	if id > 0 {
		message = "Рабочий день сотрудника успешно добавлен"
	}

	log.Printf("Рабочий день сотрудника успешно добавлен: %s ", uuid.String())
	response := ResponseMsg{Id: uuid.String()}
	w.Header().Set("Content-Type", cContentTypeJson)
	json.NewEncoder(w).Encode(response)
}
