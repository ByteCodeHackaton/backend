package main

import (
	"context"
	"encoding/json"
	"log"
	"net/http"
)

func DeleteOrder(w http.ResponseWriter, r *http.Request) {
	log.Println("Request order delete..")

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
	result, err := db.ExecContext(context.Background(), `DELETE FROM orders WHERE id=?;`, uuid_)
	if err != nil {
		message = "Ошибка удаления заявки: " + err.Error()
		http.Error(w, message, http.StatusExpectationFailed) // 417
		log.Println(message)
		return
	}

	var id int64
	id, err = result.RowsAffected()
	if err != nil {
		message = "Ошибка удаления заявки: " + err.Error()
		http.Error(w, message, http.StatusExpectationFailed) // 417
		log.Println(message)
		return
	}
	if id > 0 {
		message = "Order delete succesfully!"
		log.Println(message)
	}

	response := Order{Id: uuid_}
	w.Header().Set("Content-Type", cContentTypeJson)
	json.NewEncoder(w).Encode(response)
}
