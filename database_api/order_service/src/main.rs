mod order;
mod api;
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