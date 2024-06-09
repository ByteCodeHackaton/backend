package main

import (
	"context"
	"encoding/json"
	"log"
	"net/http"
)

func PostEmployeeUpdate(w http.ResponseWriter, r *http.Request) {
	log.Println("Request employee update..")

	var emp Employee

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

	var message string
	uuid_ := r.FormValue("id")

	result, err := db.ExecContext(context.Background(), `UPDATE employees SET date=?, timework=?, fio=?, uchastok=?, smena=?, rank=?, sex=?, is_busy=?,
		phone_work=?, phone_personal=?, tab_number=?, type_work=?  WHERE id=?;`, emp.Date, emp.Timework, emp.Fio, emp.Uchastok, emp.Smena, emp.Rank,
		emp.Sex, 0, emp.Phone_work, emp.Phone_personal, emp.Tab_number, emp.Type_work, uuid_)

	if err != nil {
		message = "Ошибка изменения информации о сотруднике: " + err.Error()
		http.Error(w, message, http.StatusExpectationFailed) // 417
		log.Println(message)
		return
	}

	var id int64
	id, err = result.LastInsertId()
	if err != nil {
		message = "Ошибка изменения информации о сотруднике: " + err.Error()
		http.Error(w, message, http.StatusExpectationFailed) // 417
		log.Println(message)
		return
	}
	if id > 0 {
		message = "Изменена информация о сотруднике: " + emp.Fio
		log.Println(message)
	}

	response := Employee{Id: uuid_}
	w.Header().Set("Content-Type", cContentTypeJson)
	json.NewEncoder(w).Encode(response)
}
