use std::fmt::Display;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::{collections::HashMap, fs::File, io::BufReader, path::Path};
use logger::{error, info};
use once_cell::sync::Lazy;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use utilites::Date;
use uuid::Timestamp;
use crate::employees::{AvalibleEmployees, Employees};

//предположим что это ввсе загружено с базы данных
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
    pub place: Place,
}

impl RequestOrder
{
    pub fn new(fio: &str, path_from: &str, path_to: &str, date: Date, note: Option<String>, place: Place) -> Self
    {
        let id =  uuid::Uuid::new_v7(Timestamp::from_rfc4122(Date::now().as_naive_datetime().and_utc().timestamp() as u64, fio.len() as u16));
        Self 
        { 
            id: id.to_string(),
            fio: fio.to_owned(),
            path_from: path_from.to_owned(),
            path_to: path_to.to_owned(),
            request_date: date,
            note,
            place
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
    pub place: Place,
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

pub fn get_orders(avalible: & AvalibleEmployees) -> Vec<Order>
{
    let g = ORDERS.get_or_init(|| Arc::new(Mutex::new(vec![])));
    let guard = g.lock().unwrap();
    guard.iter().filter(|f| f.employess.iter().find(|e| *e == &avalible.id).is_some()).map(|o| o.to_owned()).collect()
}
#[cfg(test)]
mod tests
{
   
}