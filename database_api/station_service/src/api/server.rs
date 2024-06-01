

use axum::{response::IntoResponse, routing::{get, post}, Router};
use hyper::StatusCode;
use logger::debug;
use tower_http::{cors::CorsLayer, trace::{DefaultMakeSpan, TraceLayer}};
use std::{net::SocketAddr, sync::{Arc, Mutex}, collections::HashMap};

use crate::api::services;

pub async fn run_server()
{
    let app = Router::new()
        .fallback(handler_404)        
        .route("/stations", get(services::get_stations))
        .route("/path", get(services::get_stations_path))
        .route("/nearest", get(services::get_nearest_stations))
        .layer(CorsLayer::permissive())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        );
    let api_port = 8888;
    let addr = SocketAddr::from(([0, 0, 0, 0], api_port));
    debug!("Апи сервера доступно на {}", &addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler_404() -> impl IntoResponse 
{
    (StatusCode::NOT_FOUND, "Такого пути нет")
}
