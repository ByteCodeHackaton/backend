package main

type Configuration struct {
	DbPath     string `json:"dbpath" example:"metro.db"`
	HttpDomain string `json:"httpdomain" example:"localhost"`
	HttpPort   string `json:"httpport" example:"5010"`
	Version    string `json:"version" example:"1.0"`
}

type Order struct {
	Id         string `json:"id,omitempty" example:"477354"`                   // Уникальный идентификатор заявки
	Id_Pas     string `json:"id_pas,omitempty" example:"11058"`                // Уникальный идентификатор пассажира
	DateTime   string `json:"datetime,omitempty" example:"24.04.2024 7:30:00"` // Дата и время начала заявки
	Time3      string `json:"time3,omitempty" example:"07:13:52"`              // Время встречи с пассажиром и начало его сопровождения
	Time4      string `json:"time4,omitempty" example:"07:51:11"`              // Время завершения сопровождения пассажира
	Cat_pas    string `json:"cat_pas,omitempty" example:"ИЗТ"`                 // Категория пассажира
	Status     string `json:"status,omitempty" example:"Заявка закончена"`     // Статус заявки
	Tpz        string `json:"tpz,omitempty" example:"15.03.2024 22:48:43"`     // Время регистрации заявки
	INSP_SEX_M string `json:"insp_sex_m,omitempty" example:"0"`                // Количество сотрудников мужчин выделяемых на данную заявку
	INSP_SEX_F string `json:"insp_sex_f,omitempty" example:"1"`                // Количество сотрудников женщин выделяемых на данную заявку
	TIME_OVER  string `json:"time_over,omitempty" example:"00:52:20"`          // Рассчитанное примерное время на выполнение заявки
	Id_st1     string `json:"id_st1,omitempty" example:"5"`                    // ID начальной станции
	Id_st2     string `json:"id_st2,omitempty" example:"97"`                   // ID конечной станции
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
	Fio   string `json:"fio,omitempty"`
	Role  string `json:"role,omitempty"`
}
type Employee struct {
	Date           string `json:"date,omitempty"`
	Timework       string `json:"time_work,omitempty"`
	Id             string `json:"id,omitempty"` // Уникальный идентификатор сотрудника
	Fio            string `json:"fio,omitempty"`
	Uchastok       string `json:"uchastok,omitempty"`
	Smena          string `json:"smena,omitempty"`
	Rank           string `json:"rank,omitempty"`
	Sex            string `json:"sex,omitempty"`
	Phone_work     string `json:"phone_work,omitempty"`
	Phone_personal string `json:"phone_personal,omitempty"`
	Tab_number     string `json:"tab_number,omitempty"`
	Type_work      string `json:"type_work,omitempty"`
}

type Passenger struct {
	Id          string `json:"id,omitempty"`
	Fio         string `json:"fio,omitempty"`
	Phone       string `json:"phone,omitempty"`
	Category    string `json:"category,omitempty"`
	Sex         string `json:"sex,omitempty"`
	Description string `json:"description,omitempty"`
	Eks         int    `json:"eks,omitempty"`
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

type Role struct {
	Id   string `json:"id"`
	Role string `json:"role"`
}

type ResponseMsg struct {
	Id      string `json:"id,omitempty"`
	Message string `json:"message,omitempty" example:"ок"`
}

type Response struct {
	State    string     `json:"state,omitempty" example:"error"`
	Message  string     `json:"message,omitempty" example:"ок"`
	Employee []Employee `json:"details,omitempty"`
}

type ResponseOrder struct {
	State   string  `json:"state,omitempty" example:"error"`
	Message string  `json:"message,omitempty" example:"ок"`
	Order   []Order `json:"details,omitempty"`
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

type ResponseRole struct {
	State   string `json:"state,omitempty" example:"error"`
	Message string `json:"message,omitempty" example:"ок"`
	Role    []Role `json:"details,omitempty"`
}

type DocumentResponseMsg struct {
	Document_ ResponseMsg `json:"document,omitempty"`
}

type DocumentResponse struct {
	Document_ Response `json:"document,omitempty"`
}

type DocumentResponseOrder struct {
	Document_ ResponseOrder `json:"document,omitempty"`
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

type DocumentResponseRole struct {
	Document_ ResponseRole `json:"document,omitempty"`
}
