package main

import (
	"bytes"
	"database/sql"
	"encoding/json"
	"fmt"
	"io"
	"log"
	"net/http"
	"os"
	"path/filepath"
	"time"

	"github.com/gorilla/mux"
	_ "github.com/lib/pq"
	"github.com/rs/cors"
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
	configuration.DbPath = "metro.db"
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

	// регистрация сервиса на gateway
	regService()

	router := mux.NewRouter()

	router.HandleFunc(configuration.HttpDomain+"/employee/set", PostEmployeeSet).Methods("POST")
	router.HandleFunc(configuration.HttpDomain+"/employee/update", PostEmployeeUpdate).Methods("POST")
	router.HandleFunc(configuration.HttpDomain+"/employee/list", GetEmployeeList).Methods("GET")
	router.HandleFunc(configuration.HttpDomain+"/employee/delete", DeleteEmployee).Methods("DELETE")
	router.HandleFunc(configuration.HttpDomain+"/passenger/set", PostPassengerSet).Methods("POST")
	router.HandleFunc(configuration.HttpDomain+"/passenger/update", PostPassengerUpdate).Methods("POST")
	router.HandleFunc(configuration.HttpDomain+"/passenger/list", GetPassengerList).Methods("GET")
	router.HandleFunc(configuration.HttpDomain+"/passenger", GetPassenger).Methods("GET")
	router.HandleFunc(configuration.HttpDomain+"/passenger/delete", DeletePassenger).Methods("DELETE")
	router.HandleFunc(configuration.HttpDomain+"/category/set", PostCategorySet).Methods("POST")
	router.HandleFunc(configuration.HttpDomain+"/category/list", GetCategoryList).Methods("GET")
	router.HandleFunc(configuration.HttpDomain+"/uchastok/list", GetUchastokList).Methods("GET")
	router.HandleFunc(configuration.HttpDomain+"/rank/list", GetRankList).Methods("GET")
	router.HandleFunc(configuration.HttpDomain+"/account", GetAccount).Methods("GET")
	router.HandleFunc(configuration.HttpDomain+"/account/exist", GetAccountExist).Methods("GET")
	router.HandleFunc(configuration.HttpDomain+"/order/set", PostOrderSet).Methods("POST")
	router.HandleFunc(configuration.HttpDomain+"/order/list", GetOrderList).Methods("GET")
	router.HandleFunc(configuration.HttpDomain+"/order/delete", DeleteOrder).Methods("DELETE")
	router.HandleFunc(configuration.HttpDomain+"/order/state/list", GetOrderStateList).Methods("GET")
	router.HandleFunc(configuration.HttpDomain+"/role/list", GetRoleList).Methods("GET")

	log.Println("Init router handlers...")

	c := cors.New(cors.Options{
		AllowedOrigins:   []string{"http://localhost:5173"}, // All origins
		AllowedMethods:   []string{"GET", "POST"},           // Allowing only get, just an example
		AllowCredentials: true,
	})

	// server := &http.Server{
	// 	Handler:      router,
	// 	Addr:         configuration.HttpPort,
	// 	WriteTimeout: 60 * time.Second,
	// 	ReadTimeout:  60 * time.Second,
	// }

	server := &http.Server{
		Handler:      c.Handler(router),
		Addr:         configuration.HttpPort,
		WriteTimeout: 60 * time.Second,
		ReadTimeout:  60 * time.Second,
	}

	log.Printf("Starting DepTrans API Service v.%s on port%s", configuration.Version, configuration.HttpPort)
	log.Fatal(server.ListenAndServe())
}

func regService() {
	ex, err := os.Executable()
	if err != nil {
		log.Fatal(err)
	}
	exPath := filepath.Dir(ex)
	fmt.Println(exPath)

	filename, err := os.Open(exPath + "/" + "init.json")
	//filename, err := os.Open("init.json")
	if err != nil {
		log.Fatal(err)
	}
	defer filename.Close()

	data, err := io.ReadAll(filename)
	if err != nil {
		log.Fatal(err)
	}

	var service_ Service
	jsonErr := json.Unmarshal(data, &service_)
	if jsonErr != nil {
		log.Fatal(jsonErr)
	}
	log.Println(service_)

	data, err = json.Marshal(service_)
	if err != nil {
		fmt.Println(err)
		return
	}

	req, err := http.NewRequest("POST", "http://localhost:8080/register_service", bytes.NewBuffer(data))
	if err != nil {
		fmt.Println(err)
		return
	}
	req.Header.Set("Content-Type", "application/json")

	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		fmt.Println(err)
		return
	}
	defer resp.Body.Close()

	log.Println(resp.Status)
}
