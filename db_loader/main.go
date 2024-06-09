package main

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"io"
	"log"
	"os"
	"strconv"

	"github.com/samborkent/uuidv7"
	_ "modernc.org/sqlite"
)

type MCDStation struct {
	GlobalId int64  `json:"global_id" example:"1058907003"`
	Value    string `json:"value" example:"Тимирязевская"`
}

type AeroexpressStation struct {
	GlobalId int64  `json:"global_id" example:"1508979119"`
	Value    string `json:"value" example:"Аэропорт Внуково"`
}

type RailwayStation struct {
	GlobalId int64  `json:"global_id" example:"1508979807"`
	Value    string `json:"value" example:"Павелецкий вокзал"`
}

type RailwayTerminal struct {
	GlobalId int64  `json:"global_id" example:"1058615635"`
	Value    string `json:"value" example:"Павелецкий"`
}

type Station struct {
	ID       int    `json:"ID" example:"136"`
	Station  string `json:"Station" example:"Третьяковская"`
	Line     string `json:"Line" example:"Калининская линия"`
	AdmArea  string `json:"AdmArea" example:"Центральный административный округ"`
	District string `json:"District" example:"район Замоскворечье"`
	// MCDStation_         []MCDStation         `json:"MCDStation,omitempty"`
	// AeroexpressStation_ []AeroexpressStation `json:"AeroexpressStation,omitempty"`
	// RailwayStation_     []RailwayStation     `json:"RailwayStation,omitempty"`
	// RailwayTerminal_    []RailwayTerminal    `json:"RailwayTerminal,omitempty"`
	ObjectStatus string `json:"ObjectStatus" example:"действует"`
	GlobalId     int    `json:"global_id" example:"58701962"`
}

type Employee struct {
	Date     string `json:"date"`
	Timework string `json:"time_work"`
	Id       string `json:"id"`
	Fio      string `json:"fio"`
	Uchastok string `json:"uchastok"`
	Smena    string `json:"smena"`
	Rank     string `json:"rank"`
	Sex      string `json:"sex"`
}

type Role struct {
	Role string `json:"role"`
}

var db *sql.DB

func initDatabase(dbPath string) error {
	var err error
	db, err = sql.Open("sqlite", dbPath)
	if err != nil {
		return err
	}
	return nil
}

func main() {
	filename, err := os.Open("data-1488-2024-05-06.json")
	if err != nil {
		log.Fatal(err)
	}
	defer filename.Close()

	data, err := io.ReadAll(filename)

	if err != nil {
		log.Fatal(err)
	}

	var result []Station
	//var result interface{}

	jsonErr := json.Unmarshal(data, &result)

	if jsonErr != nil {
		log.Fatal(jsonErr)
	}
	fmt.Println(result)

	///////
	dbPath := "../db/metro.db"
	err = initDatabase(dbPath)
	if err != nil {
		log.Fatal("error initializing DB connection: ", err)
	}
	err = db.Ping()
	if err != nil {
		log.Fatal("error initializing DB connection: ping error: ", err)
	}
	fmt.Println("database initialized..")
	///////

	for i := 0; i <= len(result)-1; i++ {
		addStationToDb(result[i].ID, result[i].Station, result[i].Line, result[i].AdmArea, result[i].District, result[i].ObjectStatus, int64(result[i].GlobalId))
		fmt.Println("Add station - ", result[i].Station)

		id, _ := addLineToDb(result[i].Line)
		if id > 0 {
			fmt.Println("Add line - ", result[i].Line)
		}

		id, _ = addDistrictToDb(result[i].District)
		if id > 0 {
			fmt.Println("Add district - ", result[i].District)
		}

		id, _ = addAdmAreasToDb(result[i].AdmArea)
		if id > 0 {
			fmt.Println("Add admarea - ", result[i].AdmArea)
		}
	}

	updateEmployees()
	// loadEmployees()
	// loadRoles()
}

func updateEmployees() {
	role_ := "018ffd91-fcc5-70a9-bf36-f67ca6df0ca0"
	_, err := db.ExecContext(context.Background(), `UPDATE employees SET id_role=?;`, role_)
	if err != nil {
		message := "Ошибка изменения информации о сотруднике: " + err.Error()
		fmt.Println(message)
		return
	}
}

func loadRoles() {
	filename, err := os.Open("roles.json")
	if err != nil {
		log.Fatal(err)
	}
	defer filename.Close()

	data, err := io.ReadAll(filename)

	if err != nil {
		log.Fatal(err)
	}

	var result []Role
	jsonErr := json.Unmarshal(data, &result)

	if jsonErr != nil {
		log.Fatal(jsonErr)
	}

	for i := 0; i <= len(result)-1; i++ {
		if err != nil {
			fmt.Println("Parameter transfer error!")
			return
		}

		role_uuid := uuidv7.New()
		fmt.Println("New UUID -", role_uuid.String())
		id, _ := addRolesToDb(role_uuid.String(), result[i].Role)
		if id > 0 {
			fmt.Println("Add role - ", result[i].Role)
		}
	}
}

func loadEmployees() {
	// employee ->
	// filenameStr := "fio.txt"
	// data, err = os.ReadFile(filenameStr)
	// if err != nil {
	// 	fmt.Println(err)
	// 	return
	// }
	// fmt.Println("Read", filenameStr)
	// lines := strings.Split(string(data), "\n")

	// for _, line := range lines {

	// 	employee := strings.Split(strings.ReplaceAll(line, "\r", ""), " ")
	// 	fio := employee[2] + " " + employee[0] + " " + employee[1]

	// 	id, _ := addEmployeeToDb(fio, 0)
	// 	if id > 0 {
	// 		fmt.Println("Add employee - ", fio)
	// 	}
	// }
	// <-
	filename, err := os.Open("employees.json")
	if err != nil {
		log.Fatal(err)
	}
	defer filename.Close()

	data, err := io.ReadAll(filename)

	if err != nil {
		log.Fatal(err)
	}

	var result []Employee
	jsonErr := json.Unmarshal(data, &result)

	if jsonErr != nil {
		log.Fatal(jsonErr)
	}

	///////
	dbPath := "../db/metro.db"
	err = initDatabase(dbPath)
	if err != nil {
		log.Fatal("error initializing DB connection: ", err)
	}
	err = db.Ping()
	if err != nil {
		log.Fatal("error initializing DB connection: ping error: ", err)
	}
	fmt.Println("database initialized..")
	///////

	for i := 0; i <= len(result)-1; i++ {
		//Id, err := strconv.Atoi(result[i].Id)
		if err != nil {
			fmt.Println("Parameter transfer error!")
			return
		}

		uuid := uuidv7.New()
		fmt.Printf("New UUID: %s ", uuid.String())
		addEmployeeToDb(result[i].Date, result[i].Timework, uuid.String(), result[i].Fio, result[i].Uchastok, result[i].Smena, result[i].Rank, result[i].Sex)
		fmt.Println("Add employee - ", result[i].Fio)

		uchastok_uuid := uuidv7.New()
		id, _ := addUchastokToDb(uchastok_uuid.String(), result[i].Uchastok)
		if id > 0 {
			fmt.Println("Add uchastok - ", result[i].Uchastok)
		}

		rank_uuid := uuidv7.New()
		id, _ = addRankToDb(rank_uuid.String(), result[i].Rank)
		if id > 0 {
			fmt.Println("Add rank - ", result[i].Uchastok)
		}

		if result[i].Rank == "ЦУ" {
			login := "loginQWE@" + strconv.Itoa(i)
			pass := "passQWE@" + strconv.Itoa(i)
			id, _ = addAccountsToDb(uuid.String(), login, pass)
			if id > 0 {
				fmt.Println("Add accounts for UUID - ", uuid.String())
			}
		}
	}
}

func addEmployeeToDb(date string, timework string, uuid string, fio string, uchastok string, smena string, rank string, sex string) (int64, error) {
	result, err := db.ExecContext(context.Background(), `INSERT INTO employees (date, timework, id, fio, uchastok, smena, rank, sex, is_busy) VALUES
		(?, ?, ?, ?, ?, ?, ?, ?, ?);`, date, timework, uuid, fio, uchastok, smena, rank, sex, 0)
	if err != nil {
		return 0, err
	}
	var id_ int64
	id_, err = result.LastInsertId()
	if err != nil {
		return 0, err
	}
	return id_, nil
}

// func addEmployeeToDb(fio string, is_busy int) (int64, error) {
// 	uuid := uuid.NewV4()
// 	fmt.Printf("New UUID: %s ", uuid)
// 	result, err := db.ExecContext(context.Background(), `INSERT INTO employees (id, fio, is_busy) VALUES (?, ?, ?);`,
// 		uuid, fio, is_busy)
// 	if err != nil {
// 		return 0, err
// 	}
// 	var id_ int64
// 	id_, err = result.LastInsertId()
// 	if err != nil {
// 		return 0, err
// 	}
// 	return id_, nil
// }

func addRolesToDb(uuid string, role string) (int64, error) {
	result, err := db.ExecContext(context.Background(), `INSERT INTO roles_dictionary (id, role) VALUES (?, ?);`, uuid, role)
	if err != nil {
		return 0, err
	}
	var id_ int64
	id_, err = result.LastInsertId()
	if err != nil {
		return 0, err
	}
	return id_, nil
}

func addAccountsToDb(uuid string, login string, pass string) (int64, error) {
	result, err := db.ExecContext(context.Background(), `INSERT INTO accounts (id_employee, login, password) VALUES (?, ?, ?);`, uuid, login, pass)
	if err != nil {
		return 0, err
	}
	var id_ int64
	id_, err = result.LastInsertId()
	if err != nil {
		return 0, err
	}
	return id_, nil
}

func addRankToDb(uuid string, rank string) (int64, error) {
	result, err := db.ExecContext(context.Background(), `INSERT INTO rank_dictionary (id, rank) VALUES (?, ?);`, uuid, rank)
	if err != nil {
		return 0, err
	}
	var id_ int64
	id_, err = result.LastInsertId()
	if err != nil {
		return 0, err
	}
	return id_, nil
}

func addUchastokToDb(uuid string, uchastok string) (int64, error) {
	result, err := db.ExecContext(context.Background(), `INSERT INTO uchastok_dictionary (id, uchastok) VALUES (?, ?);`, uuid, uchastok)
	if err != nil {
		return 0, err
	}
	var id_ int64
	id_, err = result.LastInsertId()
	if err != nil {
		return 0, err
	}
	return id_, nil
}

func addStationToDb(id int, station string, line string, adm_area string, district string, object_status string, global_id int64) (int64, error) {
	result, err := db.ExecContext(context.Background(), `INSERT INTO station (id, station, line, adm_area, district, object_status, global_id) VALUES (?, ?, ?, ?, ?, ?, ?);`,
		id, station, line, adm_area, district, object_status, global_id)
	if err != nil {
		return 0, err
	}
	var id_ int64
	id_, err = result.LastInsertId()
	if err != nil {
		return 0, err
	}
	return id_, nil
}

func addLineToDb(line string) (int64, error) {
	result, err := db.ExecContext(context.Background(), `INSERT INTO lines_dictionary (line) VALUES (?);`, line)
	if err != nil {
		return 0, err
	}
	var id_ int64
	id_, err = result.LastInsertId()
	if err != nil {
		return 0, err
	}
	return id_, nil
}

func addDistrictToDb(district string) (int64, error) {
	result, err := db.ExecContext(context.Background(), `INSERT INTO district_dictionary (district) VALUES (?);`, district)
	if err != nil {
		return 0, err
	}
	var id_ int64
	id_, err = result.LastInsertId()
	if err != nil {
		return 0, err
	}
	return id_, nil
}

func addAdmAreasToDb(adm_area string) (int64, error) {
	result, err := db.ExecContext(context.Background(), `INSERT INTO admareas_dictionary (admarea) VALUES (?);`, adm_area)
	if err != nil {
		return 0, err
	}
	var id_ int64
	id_, err = result.LastInsertId()
	if err != nil {
		return 0, err
	}
	return id_, nil
}
