use std::{collections::HashMap, result};

use axum::{extract::Query, Json};
use serde_derive::Deserialize;
use crate::{MetroPath, Nearest, Station};
use super::{error::AppError, response::Response};

#[derive(Debug, Deserialize)]
pub struct FullPathParams
{
    pub from: String,
    pub to: String
}
//#[axum_macros::debug_handler]
///http://localhost:8888/path?from=nd89811596&to=nd77715428
pub async fn get_stations_path(Query(params): Query<FullPathParams>) -> Json<super::response::Response::<MetroPath>>
{
    let result = crate::find_path(&params.from, &params.to);
    if let Ok(p) = result
    {
        super::response::Response::new(p)
    }
    else
    {
        super::response::Response::<MetroPath>::from_err(result.err().unwrap())
    }
}

#[derive(Debug, Deserialize)]
pub struct NearestParams
{
    pub id: String,
    pub time: u32
}
//#[axum_macros::debug_handler]
///http://localhost:8888/nearest?id=nd89811596&time=10
pub async fn get_nearest_stations(Query(params): Query<NearestParams>) -> Json<super::response::Response::<Vec<Nearest>>>
{
    let result = crate::find_nearest(&params.id, params.time);
    if let Ok(p) = result
    {
        super::response::Response::new(p)
    }
    else
    {
        super::response::Response::<Vec<Nearest>>::from_err(result.err().unwrap())
    }
}
//#[axum_macros::debug_handler]
///http://localhost:8888/stations
pub async fn get_stations() -> Json<super::response::Response::<Vec<Station>>>
{
    let stations = crate::get_stations();
    if let Ok(s) = stations
    {
        super::response::Response::new(s)
    }
    else
    {
        super::response::Response::<Vec<Station>>::from_err(stations.err().unwrap())
    }
}