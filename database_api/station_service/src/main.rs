mod draw;
mod nearest;
mod operations;
mod api;
use std::net::SocketAddr;

use api::run_server;
use logger::{debug, StructLogger};
pub use operations::{find_nearest, find_path, get_stations};
pub use nearest::Nearest;
mod metro_path;
pub use metro_path::MetroPath;
mod station;
pub use station::{Station};
mod metro_graph;
pub use metro_graph::{GRAPH, MetroGraph};


#[tokio::main]
async fn main()
{
    StructLogger::initialize_logger();
    let reg_service_addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    //регистрация эндпоинтов на гейтвее
    let reg = service_registrator::ServiceConfig::new("subway", "localhost:8888")
    .add_endpoint("/nearest", false)
    .add_endpoint("/stations", false)
    .add_endpoint("/path", false)
    .register(reg_service_addr).await;
    logger::debug!("{:?}", reg);
    run_server().await;
}