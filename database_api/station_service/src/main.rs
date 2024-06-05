mod draw;
mod nearest;
mod operations;
mod api;
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
    run_server().await;
}

#[cfg(test)]
mod tests
{
    use logger::info;

    
}