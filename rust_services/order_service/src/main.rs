mod order;
mod api;
mod db;
mod error;
mod http;
mod operations;
mod employees;
mod work_day;
use api::run_server;
use logger::{debug, StructLogger};


#[tokio::main]
async fn main()
{
    StructLogger::initialize_logger();
    run_server().await;
}

// #[cfg(test)]
// mod tests
// {
//     use logger::info;

    
// }