package main

import (
	"context"
	"encoding/json"
	"log"
	"net/http"

	"github.com/samborkent/uuidv7"
)

func PostCategorySet(w http.ResponseWriter, r *http.Request) {
	log.Println("Request category set..")

	category_ := r.FormValue("cat")

	err := initDatabase(configuration.DbPath)
	if err != nil {
		log.Fatal("error initializing DB connection: ", err)
	}
	err = db.Ping()
	if err != nil {
		log.Fatal("error initializing DB connection: ping error: ", err)
	}
	log.Println("database initialized..")

	var message, state string
	uuid := uuidv7.New()

	result, err := db.ExecContext(context.Background(), `INSERT INTO category_dictionary (id, category) VALUES (?, ?);`, uuid.String(), category_)
	if err != nil {
		message = "Ошибка добавления категории пассажиров: " + err.Error()
		http.Error(w, message, http.StatusExpectationFailed) // 417
		log.Println(message)
		return
	}
	var id_ int64
	id_, err = result.LastInsertId()
	if err != nil {
		message = "Ошибка добавления категории пассажиров: " + err.Error()
		http.Error(w, message, http.StatusExpectationFailed) // 417
		log.Println(message)
		return
	}
	if id_ > 0 {
		message = "Добавлена категория пассажиров: " + category_
		log.Println(message)
		state = uuid.String()
	}

	documentResponse := Response{State: state, Message: message}
	response := DocumentResponse{Document_: documentResponse}
	w.Header().Set("Content-Type", cContentTypeJson)
	json.NewEncoder(w).Encode(response)
}
