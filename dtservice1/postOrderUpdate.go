package main

import (
	"context"
	"encoding/json"
	"log"
	"net/http"
)

func PostOrderUpdate(w http.ResponseWriter, r *http.Request) {
	log.Println("Request order update..")

	var order Order
	var message string

	err := json.NewDecoder(r.Body).Decode(&order)
	if err != nil {
		message := "Error decoding json!" + err.Error()
		http.Error(w, err.Error(), http.StatusBadRequest) // 400
		log.Println(message)
		return
	}

	err = initDatabase(configuration.DbPath)
	if err != nil {
		log.Fatal("error initializing DB connection: ", err)
	}
	err = db.Ping()
	if err != nil {
		log.Fatal("error initializing DB connection: ping error: ", err)
	}
	log.Println("database initialized..")

	result, err := db.ExecContext(context.Background(), `UPDATE orders SET id_pas=?, datetime=?, time3=?, time4=?, cat_pas=?, status=?, tpz=?, insp_sex_m=?,
		insp_sex_f=?, time_over=?, id_st1=?, id_st2=?  WHERE id=?;`, order.Id_Pas, order.DateTime, order.Time3, order.Time4, order.Cat_pas,
		order.Status, order.Tpz, order.INSP_SEX_M, order.INSP_SEX_F, order.TIME_OVER, order.Id_st1, order.Id_st2, order.Id)

	if err != nil {
		message = "Ошибка изменения информации о заявке: " + err.Error()
		http.Error(w, message, http.StatusExpectationFailed) // 417
		log.Println(message)
		return
	}

	var id int64
	id, err = result.LastInsertId()
	if err != nil {
		message = "Ошибка изменения информации о заявке: " + err.Error()
		http.Error(w, message, http.StatusExpectationFailed) // 417
		log.Println(message)
		return
	}
	if id > 0 {
		message = "Изменена информация о заявке: " + order.Id
		log.Println(message)
	}

	response := Order{Id: order.Id}
	w.Header().Set("Content-Type", cContentTypeJson)
	json.NewEncoder(w).Encode(response)
}
