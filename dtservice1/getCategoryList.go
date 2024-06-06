package main

import (
	"context"
	"encoding/json"
	"log"
	"net/http"
)

func GetCategoryList(w http.ResponseWriter, r *http.Request) {
	log.Println("Request category list..")

	err := initDatabase(configuration.DbPath)
	if err != nil {
		log.Fatal("error initializing DB connection: ", err)
	}
	err = db.Ping()
	if err != nil {
		log.Fatal("error initializing DB connection: ping error: ", err)
	}
	log.Println("database initialized..")

	var category []Category
	rows, err := db.QueryContext(context.Background(), `SELECT * FROM category_dictionary`)

	var message, state string

	switch err {
	case nil:
		for rows.Next() {
			var cat Category

			if err := rows.Scan(&cat.Id, &cat.Category); err != nil {
				message = "Error get category list: " + err.Error()
				log.Println(message)
				http.Error(w, message, http.StatusExpectationFailed) // 417
				return
			}
			category = append(category, cat)
		}
		log.Println("Get category list successfull!")
	default:
		message = "Error get category list: " + err.Error()
		log.Println(message)
		http.Error(w, message, http.StatusExpectationFailed) // 417
		return
	}
	defer rows.Close()

	if len(category) == 0 {
		message = "Category list is empty!"
		http.Error(w, message, http.StatusNoContent) // 204
		return
	}

	response := DocumentResponseCategory{Document_: ResponseCategory{State: state, Message: message, Category: category}}
	w.Header().Set("Content-Type", cContentTypeJson)
	json.NewEncoder(w).Encode(response)
}
