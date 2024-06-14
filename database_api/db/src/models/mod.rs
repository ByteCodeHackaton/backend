mod connection;
mod operations;
mod orders;
//pub use operations::{Id, Operations, CountRequest, IdSelector, from_json, SortingOrder, Selector, QuerySelector};
use connection::get_connection;

///Создание если не существует база данных
pub async fn initialize_db()
{
    // let _cr1 = AddresseTable::create().await;
    // let _cr2 = PacketsTable::create().await;
    let r = "";
}