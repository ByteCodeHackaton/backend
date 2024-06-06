package main

import (
	"database/sql"
	"log"
	"net/http"
	"time"

	"github.com/gorilla/mux"
	_ "github.com/lib/pq"
	_ "modernc.org/sqlite"
)

const (
	cContentTypeJson = "application/json; charset=utf-8"
)

var (
	configuration = Configuration{}
)

var db *sql.DB

func initDatabase(dbPath string) error {
	var err error
	db, err = sql.Open("sqlite", dbPath)
	if err != nil {
		return err
	}
	return nil
}

func initial() {
	// можно вынести в переменные окружения ->
	configuration.HttpDomain = "/api/v1"
	configuration.HttpPort = ":5010"
	configuration.DbPath = "../db/metro.db"
	configuration.Version = "1.0"
	log.Println("Initial configuration complete!")

	// db init ->
	err := initDatabase(configuration.DbPath)
	if err != nil {
		log.Fatal("error initializing DB connection: ", err)
	}
	err = db.Ping()
	if err != nil {
		log.Fatal("error initializing DB connection: ping error: ", err)
	}
	log.Println("database initialized..")
	// db init <-
}

func main() {
	initial()

	router := mux.NewRouter()

	router.HandleFunc(configuration.HttpDomain+"/employee/set", PostEmployeeSet).Methods("POST")
	router.HandleFunc(configuration.HttpDomain+"/employee/list", GetEmployeeList).Methods("GET")
	router.HandleFunc(configuration.HttpDomain+"/passenger/set", PostPassengerSet).Methods("POST")
	router.HandleFunc(configuration.HttpDomain+"/passenger/list", GetPassengerList).Methods("GET")
	router.HandleFunc(configuration.HttpDomain+"/passenger", GetPassenger).Methods("GET")
	router.HandleFunc(configuration.HttpDomain+"/category/set", PostCategorySet).Methods("POST")
	router.HandleFunc(configuration.HttpDomain+"/category/list", GetCategoryList).Methods("GET")
	router.HandleFunc(configuration.HttpDomain+"/uchastok/list", GetUchastokList).Methods("GET")

	log.Println("Init router handlers...")
	server := &http.Server{
		Handler:      router,
		Addr:         configuration.HttpPort,
		WriteTimeout: 60 * time.Second,
		ReadTimeout:  60 * time.Second,
	}

	log.Printf("Starting DepTrans API Service v.%s on port%s", configuration.Version, configuration.HttpPort)
	log.Fatal(server.ListenAndServe())
}
