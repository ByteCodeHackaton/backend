package main

import (
	"context"
	"encoding/json"
	"log"
	"net/http"
)

func GetOrderStateList(w http.ResponseWriter, r *http.Request) {
	log.Println("Request order state list..")

	err := initDatabase(configuration.DbPath)
	if err != nil {
		log.Fatal("error initializing DB connection: ", err)
	}
	err = db.Ping()
	if err != nil {
		log.Fatal("error initializing DB connection: ping error: ", err)
	}
	log.Println("database initialized..")

	var state []State
	rows, err := db.QueryContext(context.Background(), `SELECT * FROM order_state_dictionary`)

	var message string

	switch err {
	case nil:
		for rows.Next() {
			var state_ State

			if err := rows.Scan(&state_.Id, &state_.State); err != nil {
				message = "Error get order state list: " + err.Error()
				log.Println(message)
				http.Error(w, message, http.StatusExpectationFailed) // 417
				return
			}
			state = append(state, state_)
		}
		log.Println("Get order state list successfull!")
	default:
		message = "Error get order state list: " + err.Error()
		log.Println(message)
		http.Error(w, message, http.StatusExpectationFailed) // 417
		return
	}
	defer rows.Close()

	if len(state) == 0 {
		message = "Order state list is empty!"
		http.Error(w, message, http.StatusNoContent) // 204
		return
	}

	response := DocumentResponseState{Document_: ResponseState{Message: message, State: state}}
	w.Header().Set("Content-Type", cContentTypeJson)
	json.NewEncoder(w).Encode(response)
}
