use hyper::Uri;
use serde::Deserialize;
use serde_json::Value;
use utilites::Date;

use crate::{employees::AvalibleEmployees, error::OrderError};

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

    pub async fn get_workers(date: &Date) -> Result<Vec<Workday>, crate::error::OrderError>
    {
        let uri: Uri = format!("http://localhost:5010/api/v1/workday/date/list?limit=1000&date={}", date.format(utilites::DateFormat::Serialize)).parse().unwrap();
        let result = crate::http::get::<Value>(uri).await;
        if result.is_err()
        {
            return Ok(vec![]);
        }
        let result = result.unwrap();
        let arr = result["document"]["details"].as_array().unwrap();
        let wdays = arr.iter().map(|v| serde_json::from_value::<Workday>(v.to_owned()).unwrap()).collect::<Vec<Workday>>();
        Ok(wdays)
    }

    pub async fn get_avalible_employees(date: &Date) -> Result<Vec<AvalibleEmployees>, crate::error::OrderError>
    {
        let wd = Workday::get_workers(date).await?;
        logger::info!("Количество сотрудников в днях {}", wd.len());
        let aval = wd.into_iter().map(|v| v.into()).collect::<Vec<AvalibleEmployees>>();
        logger::info!("Количество доступных сотрудников в днях {}", aval.len());
        Ok(aval)
    }
}


impl From<Workday> for AvalibleEmployees
{
    fn from(value: Workday) -> Self 
    {
        let station_id = if value.state_wd.is_empty()
        {
            "nd52567902"
        }
        else
        {
            &value.state_wd
        };
        AvalibleEmployees::new(&value.employee_id, value.date_work, &value.time_work, station_id)
    }
}


#[cfg(test)]
mod tests
{
    use hyper::Uri;
    use logger::StructLogger;
    use serde_json::Value;
    use utilites::Date;

    use crate::Workday;

    #[tokio::test]
    async fn test_get_wd()
    {
        StructLogger::initialize_logger();
        let date = Date::parse("2024-06-19T00:00:00").unwrap();
        let w = Workday::get_workers(&date).await.unwrap();
        assert_eq!(w.len(), 8);
        logger::info!("{:?}", &w);
        let range = w[0].work_dates_range();
        logger::info!("{:?}", range);
        
        
    }

    #[tokio::test]
    async fn test_get()
    {
        StructLogger::initialize_logger();
        let date = Date::parse("2024-06-19T00:00:00").unwrap();
        //"http://localhost:5010/api/v1/workday/date/list?limit=1000&date=2024-06-12T00:00:00";
        //"http://localhost:5010/api/v1/workday/date/list?limit=1000&date=12-06-2024T00:00:00";
        let uri: Uri = format!("http://localhost:5010/api/v1/workday/date/list?limit=1000&date={}", date.format(utilites::DateFormat::Serialize)).parse().unwrap();
        let result = crate::http::get::<Value>(uri).await;
        logger::info!("{:?}", result);
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