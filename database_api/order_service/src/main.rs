mod order;
mod api;
mod error;
mod operations;
mod employees;
use api::run_server;
use logger::{debug, StructLogger};




#[tokio::main]
async fn main()
{
    StructLogger::initialize_logger();
    run_server().await;
}

#[cfg(test)]
mod tests
{
    use logger::info;

    
}