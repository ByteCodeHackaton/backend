package main

import (
	"context"
	"encoding/json"
	"log"
	"net/http"
)

func GetRankList(w http.ResponseWriter, r *http.Request) {
	log.Println("Request rank list..")

	err := initDatabase(configuration.DbPath)
	if err != nil {
		log.Fatal("error initializing DB connection: ", err)
	}
	err = db.Ping()
	if err != nil {
		log.Fatal("error initializing DB connection: ping error: ", err)
	}
	log.Println("database initialized..")

	var rank []Rank
	rows, err := db.QueryContext(context.Background(), `SELECT * FROM rank_dictionary`)

	var message, state string

	switch err {
	case nil:
		for rows.Next() {
			var rank_ Rank

			if err := rows.Scan(&rank_.Id, &rank_.Rank); err != nil {
				message = "Error get rank list: " + err.Error()
				log.Println(message)
				http.Error(w, message, http.StatusExpectationFailed) // 417
				return
			}
			rank = append(rank, rank_)
		}
		log.Println("Get rank list successfull!")
	default:
		message = "Error get rank list: " + err.Error()
		log.Println(message)
		http.Error(w, message, http.StatusExpectationFailed) // 417
		return
	}
	defer rows.Close()

	if len(rank) == 0 {
		message = "Rank list is empty!"
		http.Error(w, message, http.StatusNoContent) // 204
		return
	}

	response := DocumentResponseRank{Document_: ResponseRank{State: state, Message: message, Rank: rank}}
	w.Header().Set("Content-Type", cContentTypeJson)
	json.NewEncoder(w).Encode(response)
}
