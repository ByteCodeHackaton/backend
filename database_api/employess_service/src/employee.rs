use std::fmt::Display;
use std::io::Write;
use std::{collections::HashMap, fs::File, io::BufReader, path::Path};
use logger::{error, info};
use once_cell::sync::Lazy;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;


#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Employee
{
    id: String,
    fio: String,
}

#[cfg(test)]
mod tests
{
   
}