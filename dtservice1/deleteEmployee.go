package main

import (
	"context"
	"encoding/json"
	"log"
	"net/http"
)

func DeleteEmployee(w http.ResponseWriter, r *http.Request) {
	log.Println("Request employee delete..")

	uuid_ := r.FormValue("id")

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
	result, err := db.ExecContext(context.Background(), `DELETE FROM employees WHERE id=?;`, uuid_)
	if err != nil {
		message = "Ошибка удаления сотрудника: " + err.Error()
		http.Error(w, message, http.StatusExpectationFailed) // 417
		log.Println(message)
		return
	}

	var id int64
	id, err = result.RowsAffected()
	if err != nil {
		message = "Ошибка удаления сотрудника: " + err.Error()
		http.Error(w, message, http.StatusExpectationFailed) // 417
		log.Println(message)
		return
	}
	if id > 0 {
		message = "Employee delete succesfully!"
	}

	response := Employee{Id: uuid_}
	w.Header().Set("Content-Type", cContentTypeJson)
	json.NewEncoder(w).Encode(response)
}
