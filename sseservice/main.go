package main

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"log"
	"math/rand"
	"net/http"
	"time"

	_ "modernc.org/sqlite"
)

const charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"

var seededRand *rand.Rand = rand.New(rand.NewSource(time.Now().UnixNano()))

var (
	configuration = Configuration{}
)

type (
	Orders struct {
		Date    string  `json:"date,omitempty"`
		Orders_ []Order `json:"order,omitempty"`
	}
	Order struct {
		Number       int      `json:"number,omitempty"`
		StartTime    string   `json:"startTime,omitempty"`
		EndTime      string   `json:"endTime,omitempty"`
		StationStart string   `json:"stationstart,omitempty"`
		StationEnd   string   `json:"stationend,omitempty"`
		PersonsCount int      `json:"personscount,omitempty"`
		Persons      []Person `json:"person,omitempty"`
	}
	Person struct {
		Fio     string `json:"fio,omitempty"`
		Station string `json:"station,omitempty"`
	}

	DocumentInfo struct {
		Document_ Orders `json:"document"`
	}
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
	configuration.HttpDomain = "/sse"
	configuration.HttpPort = ":5000"
	log.Print("Initial configuration complete!")

	///////
	dbPath := "metro.db"
	err := initDatabase(dbPath)
	if err != nil {
		log.Fatal("error initializing DB connection: ", err)
	}
	err = db.Ping()
	if err != nil {
		log.Fatal("error initializing DB connection: ping error: ", err)
	}
	log.Print("database initialized..")
	///////
}

func main() {
	initial()
	http.HandleFunc(configuration.HttpDomain, randomHandler)

	log.Printf("Starting server on port %s", configuration.HttpPort)
	log.Fatal(http.ListenAndServe(configuration.HttpPort, nil))
}

func randomHandler(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Content-Type", "text/event-stream;charset=UTF-8")
	w.Header().Set("Cache-Control", "no-cache")
	w.Header().Set("Connection", "keep-alive")
	w.Header().Set("Access-Control-Allow-Origin", "*")

	// Send an Intial Connection Response
	fmt.Fprint(w, "data: Connection\n\n")
	flusher, ok := w.(http.Flusher)
	if !ok {
		http.Error(w, "Streaming not supported", http.StatusInternalServerError)
		return
	}

	for {

		var order_ []Order
		var person_ []Person
		// имитация данных, надо переделать, брать из бд или из api ->
		countOrder := rand.Intn(15) + 1

		date_ := time.Now().Format("2006-01-02")

		for i := 0; i < countOrder; i++ {

			stationstart_, _ := getStationFromDb(327)
			stationend_, _ := getStationFromDb(327)
			//fio_ := generateRandomString(10)
			//station_ := generateRandomString(20)

			person_ = nil
			countPersons := rand.Intn(3) + 1
			for j := 0; j < countPersons; j++ {
				fio_, _ := getEmployeeFromDb(49)
				station_, _ := getStationFromDb(327)
				person_ = append(person_,
					Person{
						Fio:     fio_,
						Station: station_})
			}

			order_ = append(order_,
				Order{
					Number:       int(rand.Int31() % 53),
					StartTime:    time.Now().Format("2006-01-02"),
					EndTime:      time.Now().Format("2006-01-02"),
					StationStart: stationstart_,
					StationEnd:   stationend_,
					PersonsCount: countPersons,
					Persons:      person_})

		}
		// имитация данных, надо переделать, брать из бд или из api <-

		response := DocumentInfo{Document_: Orders{Date: date_, Orders_: order_}}

		jsonData, err := json.Marshal(response)
		if err != nil {
			fmt.Println("Error marshalling JSON:", err)
			continue
		}

		fmt.Fprintf(w, "data: %s\n\n", jsonData)
		flusher.Flush()
		time.Sleep(1 * time.Second)

		if f, ok := w.(http.Flusher); ok {
			f.Flush()
		}
		time.Sleep(10 * time.Second)
	}
}

func getStationFromDb(length int) (string, error) {
	id := seededRand.Intn(length) + 1
	var station = ""

	row := db.QueryRowContext(context.Background(), `SELECT station FROM station WHERE id=?`, id)
	err := row.Scan(&station)
	if err != nil {
		return station, err
	}
	return station, nil
}

func getEmployeeFromDb(length int) (string, error) {
	//id := 0 //seededRand.Intn(length) + 51
	//row := db.QueryRowContext(context.Background(), `SELECT fio WHERE id_busy=?`, id)
	var fio string
	row := db.QueryRowContext(context.Background(), `SELECT fio FROM employees ORDER BY RANDOM() LIMIT 1`)
	err := row.Scan(&fio)
	if err != nil {
		return "", err
	}
	return fio, nil
}

func generateRandomString(length int) string {
	b := make([]byte, length)
	for i := range b {
		b[i] = charset[seededRand.Intn(len(charset))]
	}
	return string(b)
}
