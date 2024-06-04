package main

import (
	"context"
	"encoding/json"
	"log"
	"net/http"
)

func PostEmployeeSet(w http.ResponseWriter, r *http.Request) {
	log.Println("Request employee set..")

	// token := r.FormValue("token")
	// if len(token) == 0 {
	// 	message := "Token not found!"
	// 	http.Error(w, message, http.StatusUnauthorized) // 401
	// 	log.Warn(message)
	// 	return
	// }

	// terminal := Term{}.getValue(token)
	// if !terminal.isValidWebOr1C() {
	// 	message := "Error token not web or 1c!"
	// 	http.Error(w, message, http.StatusNotFound) // 404
	// 	log.Warning(message)
	// 	return
	// }

	// if isNotContentType(r.Header.Get("Content-Type")) {
	// 	message := "Error unaccepted Content-Type!"
	// 	http.Error(w, message, http.StatusUnsupportedMediaType) // 415
	// 	log.Warning(message)
	// 	return
	// }

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

	var message, state string
	result, err := db.ExecContext(context.Background(), `INSERT INTO employees (date, timework, id, fio, uchastok, smena, rank, sex, is_busy) VALUES
		(?, ?, ?, ?, ?, ?, ?, ?, ?);`, emp.Date, emp.Timework, emp.Id, emp.Fio, emp.Uchastok, emp.Smena, emp.Rank, emp.Sex, 0)
	if err != nil {
		message = "Ошибка добавления сотрудника: " + err.Error()
		http.Error(w, message, http.StatusExpectationFailed) // 417
		log.Println(message)
		return
	}
	var id_ int64
	id_, err = result.LastInsertId()
	if err != nil {
		message = "Ошибка добавления сотрудника: " + err.Error()
		http.Error(w, message, http.StatusExpectationFailed) // 417
		log.Println(message)
		return
	}
	if id_ > 0 {
		message = "Добавлен сотрудник: " + emp.Fio
	}

	log.Printf("Работник %s зарегистрирован: ", emp.Fio)
	documentResponse := Response{State: state, Message: message}
	response := DocumentResponse{Document_: documentResponse}
	w.Header().Set("Content-Type", cContentTypeJson)
	json.NewEncoder(w).Encode(response)
}
