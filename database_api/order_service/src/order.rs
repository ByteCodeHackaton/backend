use std::fmt::Display;
use std::io::Write;
use std::{collections::HashMap, fs::File, io::BufReader, path::Path};
use logger::{error, info};
use once_cell::sync::Lazy;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Palce
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
pub struct Order
{
    id: String,
    fio: String,
    // from node id
    path_from: String,
    // to node id
    path_to: String,
    // date
    request_date: utilites::Date,
    average_path_time: u32,
    note: Option<String>,
    place: Palce,
    is_confirmed: bool
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ConfirmedOrder
{
    id: String,
    order_id: String,
    start_work: utilites::Date,
    end_work: utilites::Date,
    ///id сотрудников
    employess: Vec<String>,
}


#[cfg(test)]
mod tests
{
   
}