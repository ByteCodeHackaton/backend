package main

import (
	"context"
	"encoding/json"
	"log"
	"net/http"
)

func GetUchastokList(w http.ResponseWriter, r *http.Request) {
	log.Println("Request uchastok list..")

	err := initDatabase(configuration.DbPath)
	if err != nil {
		log.Fatal("error initializing DB connection: ", err)
	}
	err = db.Ping()
	if err != nil {
		log.Fatal("error initializing DB connection: ping error: ", err)
	}
	log.Println("database initialized..")

	var uchastok []Uchastok
	rows, err := db.QueryContext(context.Background(), `SELECT * FROM uchastok_dictionary`)

	var message, state string

	switch err {
	case nil:
		for rows.Next() {
			var uchastok_ Uchastok

			if err := rows.Scan(&uchastok_.Id, &uchastok_.Uchastok); err != nil {
				message = "Error get uchastok list: " + err.Error()
				log.Println(message)
				http.Error(w, message, http.StatusExpectationFailed) // 417
				return
			}
			uchastok = append(uchastok, uchastok_)
		}
		log.Println("Get uchastok list successfull!")
	default:
		message = "Error get uchastok list: " + err.Error()
		log.Println(message)
		http.Error(w, message, http.StatusExpectationFailed) // 417
		return
	}
	defer rows.Close()

	if len(uchastok) == 0 {
		message = "Uchastok list is empty!"
		http.Error(w, message, http.StatusNoContent) // 204
		return
	}

	response := DocumentResponseUchastok{Document_: ResponseUchastok{State: state, Message: message, Uchastok: uchastok}}
	w.Header().Set("Content-Type", cContentTypeJson)
	json.NewEncoder(w).Encode(response)
}
