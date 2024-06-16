mod order;
mod api;
mod db;
mod error;
mod http;
mod operations;
mod employees;
mod work_day;
use std::net::SocketAddr;

use api::run_server;
use logger::{debug, StructLogger};
pub use work_day::Workday;

#[tokio::main]
async fn main()
{
    StructLogger::initialize_logger();
    let reg_service_addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    //регистрация эндпоинтов на гейтвее
    let _ = service_registrator::ServiceConfig::new("orders_service", "localhost:8889")
    .add_endpoint("/orders/request", false)
    .register(reg_service_addr).await;
    operations::add_test_workers();
    run_server().await;
}

// #[cfg(test)]
// mod tests
// {
//     use logger::info;

    
// }