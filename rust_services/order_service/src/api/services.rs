use std::{collections::HashMap, result};

use axum::{extract::{self, rejection::JsonRejection, Query}, Json};
use logger::debug;
use serde_derive::Deserialize;
use crate::order::{Order, RequestOrder};
use super::{error::AppError, response::Response};

// #[derive(Debug, Deserialize)]
// pub struct FullPathParams
// {
//     pub from: String,
//     pub to: String
// }
// //#[axum_macros::debug_handler]
// ///http://localhost:8888/path?from=nd89811596&to=nd77715428
// pub async fn get_stations_path(Query(params): Query<FullPathParams>) -> Json<super::response::Response::<MetroPath>>
// {
//     let result = crate::find_path(&params.from, &params.to);
//     if let Ok(p) = result
//     {
//         super::response::Response::new(p)
//     }
//     else
//     {
//         super::response::Response::<MetroPath>::from_err(result.err().unwrap())
//     }
// }

// #[derive(Debug, Deserialize)]
// pub struct NearestParams
// {
//     pub id: String,
//     pub time: u32
// }
// //#[axum_macros::debug_handler]
// ///http://localhost:8888/nearest?id=nd89811596&time=10
// pub async fn get_nearest_stations(Query(params): Query<NearestParams>) -> Json<super::response::Response::<Vec<Nearest>>>
// {
//     let result = crate::find_nearest(&params.id, params.time);
//     if let Ok(p) = result
//     {
//         super::response::Response::new(p)
//     }
//     else
//     {
//         super::response::Response::<Vec<Nearest>>::from_err(result.err().unwrap())
//     }
// }



pub async fn set_orders(extract::Json(payload): extract::Json<RequestOrder>) -> Json<super::response::Response::<Order>>
{
    let order = crate::operations::add_order(&payload).await;
    if let Ok(o) = order
    {
        debug!("{:?}", &o);
        super::response::Response::<Order>::new(o)
    }
    else
    {
        super::response::Response::<Order>::from_err(order.err().unwrap().to_string())
    }
    
}