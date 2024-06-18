use std::fmt::Display;
use std::io::Write;
use std::sync::{Arc};
use tokio::sync::Mutex;
use std::{collections::HashMap, fs::File, io::BufReader, path::Path};
use logger::{error, info};
use once_cell::sync::Lazy;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use utilites::Date;
use uuid::Timestamp;
use crate::employees::{AvalibleEmployees, Employees};

//предположим что это ввсе загружено с базы данных на базу данных нет времени
pub static ORDERS : once_cell::sync::OnceCell<Arc<Mutex<Vec<Order>>>> = once_cell::sync::OnceCell::new();


#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Place
{
    ///у входа
    OnEnter,
    ///у турникетов
    OnTurnstile,
    ///в вестибюле
    OnLobby,
    ///в центре зала
    OnCenter
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RequestOrder
{
    pub id: String,
    pub fio: String,
    // from node id
    pub path_from: String,
    // to node id
    pub path_to: String,
    // date
    pub request_date: utilites::Date,
    pub note: Option<String>,
    //требуемое количество сотрудников (непонятно кто это будет решать)
    pub employees_count: u32,
    pub place: Option<Place>,
}

impl RequestOrder
{
    pub fn new(fio: &str, path_from: &str, path_to: &str, date: Date, count: u32, note: Option<String>, place: Place) -> Self
    {
        let id =  uuid::Uuid::new_v7(Timestamp::from_rfc4122(Date::now().as_naive_datetime().and_utc().timestamp() as u64, fio.len() as u16));
        Self 
        { 
            id: id.to_string(),
            fio: fio.to_owned(),
            path_from: path_from.to_owned(),
            path_to: path_to.to_owned(),
            request_date: date,
            employees_count: count,
            note,
            place: Some(place)
        }
    }
}




#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Order
{
    pub id: String,
    pub fio: String,
    pub request_date: utilites::Date,
    pub path_from: String,
    // to node id
    pub path_to: String,
    pub average_path_time: u32,
    pub note: Option<String>,
    pub place: Option<Place>,
    pub start_work: utilites::Date,
    pub end_work: utilites::Date,
    ///id сотрудников
    pub employess: Vec<String>,
}

impl Order
{
    pub fn busy_time_range(&self) -> (&Date, &Date)
    {
        (&self.start_work, &self.end_work)
    }
    
}

///получение всех заявок в которых присутсвует данный сотрудник
// pub fn get_orders(avalible: & AvalibleEmployees) -> Vec<Order>
// {
//     let g = ORDERS.get_or_init(|| Arc::new(Mutex::new(vec![])));
//     let guard = g.lock().unwrap();
//     guard.iter().filter(|f| f.employess.iter().find(|e| *e == &avalible.id).is_some()).map(|o| o.to_owned()).collect()
// }
#[cfg(test)]
mod tests
{
    use serde_json::json;

    use super::RequestOrder;
    #[test]
    fn test_deserialize()
    {
        let js = json!({"id": "01901160-bf0b-7c72-ba93-2158a7694cb8",
        "fio": "Иванова И.И.",
        "path_from": "nd52567902",
        "path_to": "nd77715428",
        "request_date": "2024-06-02T09:30:00",
        "note": "Осторожнее!",
        "employees_count": 2});
        let rq = serde_json::from_value::<RequestOrder>(js);
        println!("{:?}", rq);
    }
}