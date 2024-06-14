package main

import (
	"context"
	"database/sql"
	"encoding/json"
	"log"
	"net/http"
	"strconv"
)

func GetWorkdayDateList(w http.ResponseWriter, r *http.Request) {
	log.Println("Request workday date list..")

	date_work := r.FormValue("date")
	var message string

	if len(date_work) == 0 {
		message = "Parameters not found!"
		log.Print(message)
		http.Error(w, message, http.StatusBadRequest) // 400
		return
	}

	limit_ := r.FormValue("limit")
	offset_ := r.FormValue("off")

	if len(limit_) == 0 {
		limit_ = "20"
	}

	if len(offset_) == 0 {
		offset_ = "0"
	}

	err := initDatabase(configuration.DbPath)
	if err != nil {
		log.Fatal("error initializing DB connection: ", err)
	}
	err = db.Ping()
	if err != nil {
		log.Fatal("error initializing DB connection: ping error: ", err)
	}
	log.Println("database initialized..")

	var workday []Workday
	var row *sql.Row
	var total_count, page_count int

	row = db.QueryRowContext(context.Background(), `SELECT Count(*) FROM workdays WHERE date_work=?;`, date_work)
	err = row.Scan(&total_count)
	if err != nil {
		message = "Error get count total employees: " + err.Error()
		log.Println(message)
		http.Error(w, message, http.StatusExpectationFailed) // 417
		return
	}
	limit_i, _ := strconv.Atoi(limit_)
	page_count = total_count/limit_i + 1

	rows, err := db.QueryContext(context.Background(), `SELECT * FROM workdays WHERE date_work=? LIMIT ? OFFSET ?;`, date_work, limit_, offset_)

	var id_, employee_id_, date_work_, time_work_ string
	var state_wd_, date_dop_smena_, date_ucheba_, date_change_, intern_ NullString

	switch err {
	case nil:
		for rows.Next() {

			err := rows.Scan(&id_, &employee_id_, &date_work_, &time_work_, &state_wd_.ns, &date_dop_smena_.ns, &date_ucheba_.ns, &date_change_.ns, &intern_.ns)
			if err == nil {
				workday = append(workday,
					Workday{
						Id:             id_,
						Employee_Id:    employee_id_,
						Date_work:      date_work_,
						Time_work:      time_work_,
						State_wd:       state_wd_.String(),
						Date_dop_smena: date_dop_smena_.String(),
						Date_ucheba:    date_ucheba_.String(),
						Date_change:    date_change_.String(),
						Intern:         intern_.String()})
			}
		}
		log.Println("Get workday date list successfull!")
	default:
		message = "Error get workday list: " + err.Error()
		log.Println(message)
		http.Error(w, message, http.StatusExpectationFailed) // 417
		return
	}
	defer rows.Close()

	if len(workday) == 0 {
		message = "Workday date list is empty!"
		http.Error(w, message, http.StatusNoContent) // 204
		return
	}

	documentResponse := ResponseWorkday{Total_count: total_count, Page_count: page_count, Workday: workday}
	response := DocumentResponseWorkday{Document_: documentResponse}
	w.Header().Set("Content-Type", cContentTypeJson)
	json.NewEncoder(w).Encode(response)
}
