

use axum::{response::IntoResponse, routing::{get, post}, Router};
use hyper::StatusCode;
use logger::debug;
use tower_http::{cors::CorsLayer, trace::{DefaultMakeSpan, TraceLayer}};
use std::{net::SocketAddr, sync::{Arc, Mutex}, collections::HashMap};

//use crate::api::services;

pub async fn run_server()
{
    let app = Router::new()
        .fallback(handler_404)        
        .route("/orders/request", post(super::services::set_orders))
        .route("/orders", get(super::services::get_orders))
        //http://127.0.0.1:8888/path?from=sd92939293&to=sd263626162
        //.route("/path", get(services::get_stations_path))
        //http://127.0.0.1:8888/nearest?node_id=sd92939293&time=10
        //.route("/nearest", get(services::get_nearest_stations))
        // .route("/settings/ptypes", get(SettingsService::get_packets_types))
        // .route("/settings", post(SettingsService::update_current_settings))
        // .route("/settings/defaults", get(SettingsService::get_defaults_settings))
        // //такой запрос работает
        // //http://127.0.0.1:3000/packets?source_id=6d8c1ef5-a5ea-4dd9-a97d-5ee80f0663b1&date1=2023-02-14T13:38:33&date2=2023-02-14T13:38:40
        // .route("/packets", get(PacketService::get_packets))
        // .route("/packets/visibility/:id", post(PacketService::set_visibility))
        // .route("/packets/delete/:id", post(PacketService::full_delete_packet))
        // .route("/packets/traces", get(PacketService::get_trace_list))
        // .route("/packets/traces", post(PacketService::add_trace))
        // .route("/packets/rescan/:id", post(PacketService::rescan_packet))
        // .route("/packets/traces/delete", post(PacketService::delete_trace))
        // .route("/senders", get(SendersService::get_senders))
        // .route("/senders", post(SendersService::update_sender))
        // .route("/senders/delete/:id", post(SendersService::delete_sender))
        // .route("/parser", get(ParserService::parser_status))
        // .route("/parser/transactions", get(ParserService::get_transactions))
        // .route("/parser/transactions/delete/:id", post(ParserService::delete_transaction))
        //.route("/checkpublication/:id", post(force_update_publication_status))
        //хз почему не работает вебсокет((( пришлось параллельно использовать другую либу, но я и не жалею, сделал норм на будущее
        //.route("/ws", get(WsService::ws_handler))
        .layer(CorsLayer::permissive())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        );
    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    // MessageBus::subscribe(|_f|
    // {
    //     debug!("Подписка успешна! сообщение по подписке получено");
    // });
    let api_port = 8889;
    let addr = SocketAddr::from(([0, 0, 0, 0], api_port));
    debug!("Апи сервера доступно на {}", &addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler_404() -> impl IntoResponse 
{
    (StatusCode::NOT_FOUND, "Такого пути нет")
}
