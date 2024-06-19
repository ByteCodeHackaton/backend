mod requests_table;
mod orders_table;
mod employees_table;
mod workers_table;
pub use db::Operations;
const DB_DATE_FORMAT: utilites::DateFormat = utilites::DateFormat::Serialize;
pub async fn initialize_db()
{
    let tbl = requests_table::RequestsTable::create().await;
    //let tbl = orders_table::OrdersTable::create().await;
    logger::debug!("СОздание таблицы... {:?}", tbl);
}
