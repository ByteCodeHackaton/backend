use std::fmt::Display;
use std::io::Write;
use std::{collections::HashMap, fs::File, io::BufReader, path::Path};
use logger::{error, info};
use once_cell::sync::Lazy;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;


#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Workers
{
    id: String,
    employee_id: String,
    // дата когда работник на смене
    date: utilites::Date,
    //станция на которой находиться сотрудник
    station_id: String,
    is_busy: bool
}

#[cfg(test)]
mod tests
{
   
}