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

type NullString struct {
	ns sql.NullString
}

func (ns NullInt64) Int64() int64 {
	if !ns.ni.Valid {
		ns.ni.Valid = true
		ns.ni.Int64 = 0
	}
	return ns.ni.Int64
}

func (ns NullString) String() string {
	if !ns.ns.Valid {
		ns.ns.Valid = true
		ns.ns.String = ""
	}
	return ns.ns.String
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
	var date_, timework_, id_, fio_, uchastok_, smena_, rank_, sex_ string
	var is_busy int
	var cur_station NullInt64 // по умолчанию null
	var phone_work_, phone_personal_, tab_number_, type_work_ NullString

	switch err {
	case nil:
		for rows.Next() {

			err := rows.Scan(&date_, &timework_, &id_, &fio_, &uchastok_, &smena_, &rank_, &sex_, &is_busy, &cur_station.ni,
				&phone_work_.ns, &phone_personal_.ns, &tab_number_.ns, &type_work_.ns)
			if err == nil {
				employee = append(employee,
					Employee{
						Date:           date_,
						Timework:       timework_,
						Id:             id_,
						Fio:            fio_,
						Uchastok:       uchastok_,
						Smena:          smena_,
						Rank:           rank_,
						Sex:            sex_,
						Phone_work:     phone_work_.String(),
						Phone_personal: phone_personal_.String(),
						Tab_number:     tab_number_.String(),
						Type_work:      type_work_.String()})
			}
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
