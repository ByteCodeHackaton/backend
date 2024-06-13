package main

import (
	"context"
	"encoding/json"
	"log"
	"net/http"
)

func PostWorkdayUpdate(w http.ResponseWriter, r *http.Request) {
	log.Println("Request workday update..")

	var workday Workday

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

	result, err := db.ExecContext(context.Background(), `UPDATE workdays SET employee_id=?, date_work=?, time_work=?, state_wd=?, date_dop_smena=?,
		date_ucheba=?, date_change=?, intern=? WHERE id=?;`, workday.Employee_Id, workday.Date_work, workday.Time_work, workday.State_wd,
		workday.Date_dop_smena, workday.Date_ucheba, workday.Date_change, workday.Intern, workday.Id)

	if err != nil {
		message = "Ошибка изменения информации о рабочем дне сотрудника: " + err.Error()
		http.Error(w, message, http.StatusExpectationFailed) // 417
		log.Println(message)
		return
	}
	var id_ int64
	id_, err = result.RowsAffected()
	if err != nil {
		message = "Ошибка изменения информации о рабочем дне сотрудника: " + err.Error()
		http.Error(w, message, http.StatusExpectationFailed) // 417
		log.Println(message)
		return
	}
	if id_ > 0 {
		message = "Изменена информация о рабочем дне сотрудника: " + workday.Id
		log.Println(message)
	}

	response := Passenger{Id: workday.Id}
	w.Header().Set("Content-Type", cContentTypeJson)
	json.NewEncoder(w).Encode(response)
}
