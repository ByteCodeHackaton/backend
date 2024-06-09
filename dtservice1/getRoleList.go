package main

import (
	"context"
	"encoding/json"
	"log"
	"net/http"
)

func GetRoleList(w http.ResponseWriter, r *http.Request) {
	log.Println("Request role list..")

	err := initDatabase(configuration.DbPath)
	if err != nil {
		log.Fatal("error initializing DB connection: ", err)
	}
	err = db.Ping()
	if err != nil {
		log.Fatal("error initializing DB connection: ping error: ", err)
	}
	log.Println("database initialized..")

	var role []Role
	rows, err := db.QueryContext(context.Background(), `SELECT * FROM role_dictionary`)

	var message, state string

	switch err {
	case nil:
		for rows.Next() {
			var role_ Role

			if err := rows.Scan(&role_.Id, &role_.Role); err != nil {
				message = "Error get role list: " + err.Error()
				log.Println(message)
				http.Error(w, message, http.StatusExpectationFailed) // 417
				return
			}
			role = append(role, role_)
		}
		log.Println("Get role list successfull!")
	default:
		message = "Error get role list: " + err.Error()
		log.Println(message)
		http.Error(w, message, http.StatusExpectationFailed) // 417
		return
	}
	defer rows.Close()

	if len(role) == 0 {
		message = "Role list is empty!"
		http.Error(w, message, http.StatusNoContent) // 204
		return
	}

	response := DocumentResponseRole{Document_: ResponseRole{State: state, Message: message, Role: role}}
	w.Header().Set("Content-Type", cContentTypeJson)
	json.NewEncoder(w).Encode(response)
}
