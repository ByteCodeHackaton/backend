package main

import (
	"context"
	"database/sql"
	"encoding/json"
	"log"
	"net/http"
)

type NullInt64 struct {
	ni sql.NullInt64
}

func (ns NullInt64) Int64() int64 {
	if !ns.ni.Valid {
		ns.ni.Valid = true
		ns.ni.Int64 = 0
	}
	return ns.ni.Int64
}

func GetEmployeeList(w http.ResponseWriter, r *http.Request) {
	log.Println("Request employee list..")

	err := initDatabase(configuration.DbPath)
	if err != nil {
		log.Fatal("error initializing DB connection: ", err)
	}
	err = db.Ping()
	if err != nil {
		log.Fatal("error initializing DB connection: ping error: ", err)
	}
	log.Println("database initialized..")

	var employee []Employee
	rows, err := db.QueryContext(context.Background(), `SELECT * FROM employees`)

	var message, state string
	var is_busy int
	var cur_station NullInt64 // по умолчанию null

	switch err {
	case nil:
		for rows.Next() {
			var emp Employee

			if err := rows.Scan(&emp.Date, &emp.Timework, &emp.Id, &emp.Fio, &emp.Uchastok, &emp.Smena, &emp.Rank, &emp.Sex, &is_busy, &cur_station.ni); err != nil {
				message = "Error get employees list: " + err.Error()
				log.Println(message)
				http.Error(w, message, http.StatusExpectationFailed) // 417
				return
			}
			employee = append(employee, emp)
		}
		log.Println("Get employees list successfull!")
	default:
		message = "Error get employees list: " + err.Error()
		log.Println(message)
		http.Error(w, message, http.StatusExpectationFailed) // 417
		return
	}
	defer rows.Close()

	if len(employee) == 0 {
		message = "Employees list is empty!"
		http.Error(w, message, http.StatusNoContent) // 204
		return
	}

	documentResponse := Response{State: state, Message: message, Employee: employee}
	response := DocumentResponse{Document_: documentResponse}
	w.Header().Set("Content-Type", cContentTypeJson)
	json.NewEncoder(w).Encode(response)
}
