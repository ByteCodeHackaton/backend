package main

import (
	"context"
	"database/sql"
	"encoding/json"
	"log"
	"net/http"
	"strconv"
)

func GetOrderList(w http.ResponseWriter, r *http.Request) {
	log.Println("Request order list..")

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

	var order []Order
	var message string
	var row *sql.Row
	var total_count, page_count int

	row = db.QueryRowContext(context.Background(), `SELECT Count(*) FROM orders;`)
	err = row.Scan(&total_count)
	if err != nil {
		message = "Error get count total orders: " + err.Error()
		log.Println(message)
		http.Error(w, message, http.StatusExpectationFailed) // 417
		return
	}
	limit_i, _ := strconv.Atoi(limit_)
	page_count = total_count/limit_i + 1

	rows, err := db.QueryContext(context.Background(), `SELECT * FROM orders LIMIT ? OFFSET ?`, limit_, offset_)

	var id_, id_pas_, datetime_, time3_, time4_, cat_pas_, status_, tpz_, insp_sex_m_, insp_sex_f_, time_over_, id_st1_, id_st2_ string

	switch err {
	case nil:
		for rows.Next() {

			err := rows.Scan(&id_, &id_pas_, &datetime_, &time3_, &time4_, &cat_pas_, &status_, &tpz_, &insp_sex_m_, &insp_sex_f_,
				&time_over_, &id_st1_, &id_st2_)
			if err == nil {
				order = append(order,
					Order{
						Id:         id_,
						Id_Pas:     id_pas_,
						DateTime:   datetime_,
						Time3:      time3_,
						Time4:      time4_,
						Cat_pas:    cat_pas_,
						Status:     status_,
						Tpz:        tpz_,
						INSP_SEX_M: insp_sex_m_,
						INSP_SEX_F: insp_sex_f_,
						TIME_OVER:  time_over_,
						Id_st1:     id_st1_,
						Id_st2:     id_st2_})
			}
		}
		log.Println("Get orders list successfull!")
	default:
		message = "Error get orders list: " + err.Error()
		log.Println(message)
		http.Error(w, message, http.StatusExpectationFailed) // 417
		return
	}
	defer rows.Close()

	if len(order) == 0 {
		message = "Orders list is empty!"
		http.Error(w, message, http.StatusNoContent) // 204
		return
	}

	documentResponse := ResponseOrder{Total_count: total_count, Page_count: page_count, Order: order}
	response := DocumentResponseOrder{Document_: documentResponse}
	w.Header().Set("Content-Type", cContentTypeJson)
	json.NewEncoder(w).Encode(response)
}
