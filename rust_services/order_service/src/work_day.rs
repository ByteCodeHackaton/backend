use serde::Deserialize;
use utilites::Date;

use crate::error::OrderError;

//моя структура
struct WorkdayTemplate
{
    /// Уникальный идентификатор рабочего дня сотрудника
    id: String,
    /// Уникальный идентификатор сотрудника
    employee_id: String,
    /// Дата выхода
    date_work: Date,
    /// Время работы (07:00-19:00, 08:00-20:00, 20:00-08:00, 08:00-17:00)
    time_work: Date,
    /// Статус рабочего дня
    status: String,
    /// Дополнительная смена (выход не по своему графику, дата)
    extra_shift: Option<String>,
    /// Учеба с отрывом от производства (дата от-до)                             
    education: Option<String>,
    /// Изменение времени работы (если время работы не совпадает с графиком)                                 
    custom_time: Option<String>,
    /// Стажировка (заявки только совместно с наставником)                                          
    intern: Option<String>
}

#[derive(Deserialize, Debug, Clone)]
pub struct Workday
{
    /// Уникальный идентификатор рабочего дня сотрудника
    id: String,
    /// Уникальный идентификатор сотрудника
    employee_id: String,
    /// Дата выхода
    date_work: Date,
    /// Время работы (07:00-19:00, 08:00-20:00, 20:00-08:00, 08:00-17:00)
    time_work: String,
    /// Статус рабочего дня
    state_wd: String,
    /// Дополнительная смена (выход не по своему графику, дата)
    date_dop_smena: Option<String>,
    /// Учеба с отрывом от производства (дата от-до)                             
    date_ucheba: Option<String>,
    /// Изменение времени работы (если время работы не совпадает с графиком)                                 
    date_change: Option<String>,
    /// Стажировка (заявки только совместно с наставником)                                          
    intern: Option<String>
}


impl Workday
{
    pub fn work_dates_range(&self) -> (Date, Date)
    {
        let (start_time, end_time) = self.time_work.split_once("-").unwrap();
        let (start_hour, _) = start_time.split_once(":").unwrap();
        let (end_hour, _) = end_time.split_once(":").unwrap();
        let start_date_time = self.date_work.clone().add_minutes(start_hour.parse::<i64>().unwrap() * 60);
        let end_date_time = self.date_work.clone().add_minutes(end_hour.parse::<i64>().unwrap() * 60);
        (start_date_time, end_date_time)
    }
}



// pub async fn get_work_days() -> Result<Workday, OrderError>
// {
    
// }

// "id": "01901160-bf0b-7c72-ba93-2158a7694cb8",
//         "employee_id": "018fee07-b8fb-7350-8147-d6d2925d6873",
//         "date_work": "2024-06-10T00:00:00",
//         "time_work": "07:00-19:00",
//         "state_wd": "выходной",
//         "date_dop_smena": "",
//         "date_ucheba": "",
//         "date_change": "",
//         "intern": ""