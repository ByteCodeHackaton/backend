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

pub static FREE : once_cell::sync::OnceCell<Arc<Mutex<Vec<AvalibleEmployees>>>> = once_cell::sync::OnceCell::new();
pub static ALL : once_cell::sync::OnceCell<Arc<Mutex<Vec<Employees>>>> = once_cell::sync::OnceCell::new();

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AvalibleEmployees
{
    pub id: String,
    pub employee_id: String,
    // дата когда работник на смене
    date: utilites::Date,
    //станция на которой находиться сотрудник
    pub station_id: String,
    is_busy: bool
}
///Проверяем доступен ли выделенный работник должна совпасть его текущая станция метро и дата заявки
pub fn search_by_station(id: &str, order_date: &Date) -> Option<AvalibleEmployees>
{
    let g = FREE.get().unwrap().lock().unwrap();
    g.iter().find(|s| &s.station_id == id && s.date.date_is_equalis(order_date)).cloned()
}

impl AvalibleEmployees
{
    pub fn new(emp: &Employees, date: Date, station: &str) -> Self
    {
        let id =  uuid::Uuid::new_v7(Timestamp::from_rfc4122(Date::now().as_naive_datetime().and_utc().timestamp() as u64, date.as_naive_datetime().and_utc().timestamp() as u16));
        Self 
        { 
            id: id.to_string(),
            employee_id: emp.id.clone(),
            date,
            station_id: station.to_owned(),
            is_busy: false 
        }
    }
}


#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Employees
{
    pub id: String,
    fio: String,
}
impl Employees
{
    pub fn new(fio: &str) -> Self
    {
        let id =  uuid::Uuid::new_v7(Timestamp::from_rfc4122(Date::now().as_naive_datetime().and_utc().timestamp() as u64, fio.len() as u16));
        Self
        {
            id: id.to_string(),
            fio: fio.to_owned()
        }
    }
}


#[cfg(test)]
mod tests
{
   
}