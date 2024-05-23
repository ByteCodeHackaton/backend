package main

import (
	"encoding/json"
	"fmt"
	"log"
	"math/rand"
	"net/http"
	"time"
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

func initial() {
	configuration.HttpDomain = "/sse"
	configuration.HttpPort = ":5000"
	log.Print("Initial configuration complete!")
}

func main() {
	initial()
	http.HandleFunc(configuration.HttpDomain, randomHandler)

	log.Printf("Starting server on port %s", configuration.HttpPort)
	log.Fatal(http.ListenAndServe(configuration.HttpPort, nil))
}

func randomHandler(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Content-Type", "text/event-stream")
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
		countOrder := rand.Intn(15)

		fio_ := generateRandomString(10)
		station_ := generateRandomString(20)
		date_ := time.Now().Format("2006-01-02")

		for i := 0; i < countOrder; i++ {

			countPersons := rand.Intn(4)
			for j := 0; j < countPersons; j++ {
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

func generateRandomString(length int) string {
	b := make([]byte, length)
	for i := range b {
		b[i] = charset[seededRand.Intn(len(charset))]
	}
	return string(b)
}
