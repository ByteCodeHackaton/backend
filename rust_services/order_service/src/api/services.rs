use std::{collections::HashMap, result};

use axum::{extract::{self, rejection::JsonRejection, Query, Path}, Json};
use logger::debug;
use serde_derive::Deserialize;
use crate::{order::{Order, RequestOrder, ORDERS}, Workday};
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
pub async fn get_orders() -> Json<super::response::Response::<Vec<Order>>>
{
    if let Some(orders) = ORDERS.get()
    {
        let guard = orders.lock().await;
        let orders = guard.clone();
        super::response::Response::<Vec<Order>>::new(orders)
    }
    else 
    {
        super::response::Response::<Vec<Order>>::from_err("Не найдено ни одной заявки".to_owned())
    }
}

#[derive(Deserialize)]
pub struct IdQuery
{
    id: String
}
pub async fn get_orders_by_id(id: Query<IdQuery>) -> Json<super::response::Response::<Order>>
{
    let id = &id.id;
    if let Some(orders) = ORDERS.get()
    {
        let guard = orders.lock().await;
        if let Some(order) = guard.iter().find(|f| &f.id == id)
        {
            super::response::Response::<Order>::new(order.clone())
        }
        else
        {
            super::response::Response::<Order>::from_err(["Заявка с id ", id, " не найдена"].concat())
        }
        
    }
    else 
    {
        super::response::Response::<Order>::from_err("Не найдено ни одной заявки".to_owned())
    }
}