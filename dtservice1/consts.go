package main

type Configuration struct {
	DbPath     string `json:"dbpath" example:"metro.db"`
	HttpDomain string `json:"httpdomain" example:"localhost"`
	HttpPort   string `json:"httpport" example:"5010"`
	Version    string `json:"version" example:"1.0"`
}

type Order struct {
	Id         string `json:"id" example:"477354"`                   // Уникальный идентификатор заявки
	Id_Pas     string `json:"id_pas" example:"11058"`                // Уникальный идентификатор пассажира
	DateTime   string `json:"datetime" example:"24.04.2024 7:30:00"` // Дата и время начала заявки
	Time3      string `json:"time3" example:"07:13:52"`              // Время встречи с пассажиром и начало его сопровождения
	Time4      string `json:"time4" example:"07:51:11"`              // Время завершения сопровождения пассажира
	Cat_pas    string `json:"cat_pas" example:"ИЗТ"`                 // Категория пассажира
	Status     string `json:"status" example:"Заявка закончена"`     // Статус заявки
	Tpz        string `json:"tpz" example:"15.03.2024 22:48:43"`     // Время регистрации заявки
	INSP_SEX_M string `json:"insp_sex_m" example:"0"`                // Количество сотрудников мужчин выделяемых на данную заявку
	INSP_SEX_F string `json:"insp_sex_f" example:"1"`                // Количество сотрудников женщин выделяемых на данную заявку
	TIME_OVER  string `json:"time_over" example:"00:52:20"`          // Рассчитанное примерное время на выполнение заявки
	Id_st1     string `json:"id_st1" example:"5"`                    // ID начальной станции
	Id_st2     string `json:"id_st2" example:"97"`                   // ID конечной станции
}

type EmpDocument struct {
	Emp EmpDoc `json:"document"`
}

type EmpDoc struct {
	Employee_ Employee `json:"details,omitempty"`
}

type Account struct {
	Id    string `json:"id"` // Уникальный идентификатор сотрудника
	Login string `json:"login,omitempty"`
	Pass  string `json:"pass,omitempty"`
}
type Employee struct {
	Date           string `json:"date"`
	Timework       string `json:"time_work"`
	Id             string `json:"id"` // Уникальный идентификатор сотрудника
	Fio            string `json:"fio"`
	Uchastok       string `json:"uchastok"`
	Smena          string `json:"smena"`
	Rank           string `json:"rank"`
	Sex            string `json:"sex"`
	Phone_work     string `json:"phone_work"`
	Phone_personal string `json:"phone_personal"`
	Tab_number     string `json:"tab_number"`
	Type_work      string `json:"type_work"`
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

type Uchastok struct {
	Id       string `json:"id"`
	Uchastok string `json:"uchastok"`
}

type Rank struct {
	Id   string `json:"id"`
	Rank string `json:"rank"`
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

type ResponseCategory struct {
	State    string     `json:"state,omitempty" example:"error"`
	Message  string     `json:"message,omitempty" example:"ок"`
	Category []Category `json:"details,omitempty"`
}

type ResponseUchastok struct {
	State    string     `json:"state,omitempty" example:"error"`
	Message  string     `json:"message,omitempty" example:"ок"`
	Uchastok []Uchastok `json:"details,omitempty"`
}

type ResponseRank struct {
	State   string `json:"state,omitempty" example:"error"`
	Message string `json:"message,omitempty" example:"ок"`
	Rank    []Rank `json:"details,omitempty"`
}

type DocumentResponse struct {
	Document_ Response `json:"document,omitempty"`
}

type DocumentResponsePassenger struct {
	Document_ ResponsePassenger `json:"document,omitempty"`
}

type DocumentResponseCategory struct {
	Document_ ResponseCategory `json:"document,omitempty"`
}

type DocumentResponseUchastok struct {
	Document_ ResponseUchastok `json:"document,omitempty"`
}

type DocumentResponseRank struct {
	Document_ ResponseRank `json:"document,omitempty"`
}
