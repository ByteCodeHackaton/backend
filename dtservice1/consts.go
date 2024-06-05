package main

type Configuration struct {
	DbHost     string `json:"dbhost" example:"127.0.0.1"`
	DbPort     string `json:"dbport" example:"5432"`
	DbUser     string `json:"-" example:"-"`
	DbPass     string `json:"-" example:"-"`
	DbName     string `json:"-" example:"-"`
	DbPath     string `json:"dbpath" example:"metro.db"`
	HttpDomain string `json:"httpdomain" example:"localhost"`
	HttpPort   string `json:"httpport" example:"5010"`
	Version    string `json:"version" example:"1.0"`
}

type EmpDocument struct {
	Emp EmpDoc `json:"document"`
}

type EmpDoc struct {
	Employee_ Employee `json:"details,omitempty"`
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

type Passenger struct {
	Id          string `json:"id"`
	Fio         string `json:"fio"`
	Phone       string `json:"phone"`
	Category    string `json:"category"`
	Sex         string `json:"sex"`
	Description string `json:"description"`
	Eks         int    `json:"eks"`
}

type Category struct {
	Id       string `json:"id"`
	Category string `json:"category"`
}

type Response struct {
	State    string     `json:"state,omitempty" example:"error"`
	Message  string     `json:"message,omitempty" example:"ок"`
	Employee []Employee `json:"details,omitempty"`
}

type ResponsePassenger struct {
	State     string      `json:"state,omitempty" example:"error"`
	Message   string      `json:"message,omitempty" example:"ок"`
	Passenger []Passenger `json:"details,omitempty"`
}

type DocumentResponse struct {
	Document_ Response `json:"document,omitempty"`
}

type DocumentResponsePassenger struct {
	Document_ ResponsePassenger `json:"document,omitempty"`
}
