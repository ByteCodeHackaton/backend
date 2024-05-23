package main

type Configuration struct {
	DbHost     string `json:"dbhost" example:"127.0.0.1"`
	DbPort     string `json:"dbport" example:"5432"`
	DbUser     string `json:"-" example:"-"`
	DbPass     string `json:"-" example:"-"`
	DbName     string `json:"-" example:"-"`
	HttpDomain string `json:"httpdomain" example:"localhost"`
	HttpPort   string `json:"httpport" example:"5000"`
	Sleep      int    `json:"sleep" example:"5"`
}
