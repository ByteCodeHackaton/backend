use std::{borrow::Cow, ops::Deref};

use logger::backtrace;
use serde_json::json;
use db::{anyhow, get_connection, SqliteRow, Execute, FromRow, Id, IdSelector, Operations, QuerySelector, Row, Selector, SortingOrder};
use utilites::Date;
use uuid::Uuid;

use super::DB_DATE_FORMAT;


#[derive(Debug)]
///Заявка
pub struct OrdersTable
{
    /// Уникальный идентификатор
    id: String,
    /// Уникальный идентификатор запроса на заявку
    order_id: String,
    /// id сотрудника на этой заявке
    employee_id: String,
    /// начало работы на заявке
    start_work: Date,
    /// окончание работы на заявке
    end_work: Date
}


impl<'a> Id<'a> for OrdersTable
{
    fn get_id(&'a self)-> Uuid
    {
        Uuid::parse_str(&self.id).unwrap()
    }
}

impl FromRow<'_, SqliteRow> for OrdersTable
{
    fn from_row(row: &SqliteRow) -> db::Result<Self> 
    {
        let start_work: String = row.try_get("start_work")?;
        let start_work = Date::parse(start_work).unwrap();
        let end_work: String = row.try_get("end_work")?;
        let end_work = Date::parse(end_work).unwrap();
        Ok(Self
        {
            id: row.try_get("id")?,
            order_id: row.try_get("order_id")?,
            employee_id: row.try_get("employee_id")?,
            start_work,
            end_work
        })
    }
}

impl<'a> Operations<'a> for OrdersTable
{
    fn table_name() -> &'static str 
    {
       "orders"
    }
    fn create_table() -> String 
    {  
        ["CREATE TABLE IF NOT EXISTS ", Self::table_name(), " (
            id TEXT PRIMARY KEY NOT NULL,
            order_id TEXT NOT NULL,
            employee_id TEXT NOT NULL, 
            start_work TEXT NOT NULL, 
            end_work TEXT NOT NULL
            );"].concat()
    }
    fn full_select() -> String 
    {
        ["SELECT 
        id,
        order_id,
        employee_id, 
        start_work, 
        end_work
        FROM ", Self::table_name()].concat()
    }
    async fn update(&'a self) -> anyhow::Result<()>
    {
        let mut c = get_connection().await?;
        let sql = ["UPDATE ", Self::table_name(),
        " SET 
        order_id = $2
        employee_id = $3
        start_work = $4,
        end_work = $5,
        WHERE id = $1"].concat();
        db::query(&sql)
        .bind(&self.id)
        .bind(&self.order_id)
        .bind(&self.employee_id)
        .bind(&self.start_work.format(DB_DATE_FORMAT))
        .bind(&self.end_work.format(DB_DATE_FORMAT))
        .execute(&mut c).await?;
        Ok(())
    }
   async fn select<Q: QuerySelector<'a>>(selector: &Q) -> db::anyhow::Result<Vec<Self>> 
   {
        let mut c = get_connection().await?;
        let query = selector.query();
        let mut res = db::query_as::<_, Self>(&query.0);
        if let Some(params) = query.1
        {
            for p in params
            {
                res = res.bind(p);
            }
        };
        let r = res.fetch_all(&mut c)
        .await?;
        Ok(r)
   }

    async fn add_or_replace(&'a self) -> db::anyhow::Result<()>
    {
        let mut c = get_connection().await?;
        let sql = ["INSERT OR REPLACE INTO ", Self::table_name(), 
        " (
         id,
        order_id,
        employee_id, 
        start_work, 
        end_work) 
        VALUES ($1, $2, $3, $4, $5)"].concat();
        db::query(&sql)
        .bind(&self.id)
        .bind(&self.order_id)
        .bind(&self.employee_id)
        .bind(&self.start_work.format(DB_DATE_FORMAT))
        .bind(&self.end_work.format(DB_DATE_FORMAT))
        .execute(&mut c).await?;
        Ok(())
    }
    async fn add_or_ignore(&'a self) -> anyhow::Result<()>
    {
        let mut c = get_connection().await?;
        let sql = ["INSERT OR IGNORE INTO ", Self::table_name(), 
        " (
        id,
        order_id,
        employee_id, 
        start_work, 
        end_work) 
        VALUES ($1, $2, $3, $4, $5)"].concat();
        db::query(&sql)
        .bind(&self.id)
        .bind(&self.order_id)
        .bind(&self.employee_id)
        .bind(&self.start_work.format(DB_DATE_FORMAT))
        .bind(&self.end_work.format(DB_DATE_FORMAT))
        .execute(&mut c).await?;
        Ok(())
    }
}

impl OrdersTable
{
   
}

#[cfg(test)]
mod tests
{
    use db::{Operations, QuerySelector, Selector};
    use logger::StructLogger;

    use crate::db::orders_table::OrdersTable;


    // use super::{Operations, Selector, QuerySelector};
    #[tokio::test]
    async fn test_add_order()
    {
        StructLogger::initialize_logger();
        super::super::initialize_db().await;
        let id = "d428fc2b-db42-4737-a211-414ffc41809d".to_string();
        let o_id = "fa77873a-92f7-42d1-9a19-a79e862b3fc1".to_owned();
        let e_id = "fa77873a-ffff-42d1-9a19-a79e862b3fc1".to_owned();
        let user = OrdersTable
        {
            id: id.clone(),
            order_id: o_id.clone(),
            employee_id: e_id.clone(),
            start_work: utilites::Date::new_date_time(14, 6, 2024, 18, 40, 23),
            end_work: utilites::Date::new_date_time(14, 6, 2024, 19, 28, 0),
        };
        let re = OrdersTable::add_or_replace(&user).await;
        logger::info!("{:?}", re);
        let station_id = "NB945894".to_owned();
        let selector_1 = Selector::new(&OrdersTable::full_select());
        println!("{}", selector_1.query().0);
        let select = OrdersTable::select(&selector_1).await.unwrap();
        println!("{:?}", &select);
        assert!(select.len() == 1);
    }
    // #[tokio::test]
    // async fn test_add_user()
    // {
    //     logger::StructLogger::initialize_logger();
    //     let paging : Vec<String> = PacketsTable::get_with_offset(3, 0, None).await.unwrap().into_iter().map(|m| m.packet_info.delivery_time).collect();
    //     logger::debug!("{:?}", paging);
    // }

    // #[tokio::test]
    // async fn test_json_select()
    // {
    //     super::initialize().await;
    //     let selector_1 = Selector::new(&super::UsersTable::full_select())
    //     .add_json_param("phones->'is_main'", &false);
    //     println!("{}", selector_1.query().0);
    //     let select = super::UsersTable::select(&selector_1).await.unwrap();
    //     println!("{:?}", &select);
    //     assert!(select.len() == 1);
    //     //let _ = super::DiseasesTable::delete(&d).await;
    //     //assert!(super::DiseasesTable::select(&selector_1).await.unwrap().len() == 0);
    // }

    // #[tokio::test]
    // async fn test_diseases_user_select()
    // {
    //     logger::StructLogger::initialize_logger();
    //     let _ = super::initialize().await;
    //     let select = UsersTable::get_current_diseases_users().await.unwrap();
    //     assert!(select.len() == 1);
    //     //let _ = super::DiseasesTable::delete(&d).await;
    //     //assert!(super::DiseasesTable::select(&selector_1).await.unwrap().len() == 0);
    // }
    // #[tokio::test]
    // async fn test_vacations_user_select()
    // {
    //     let _ = super::initialize().await;
    //     let select = UsersTable::get_users_status().await.unwrap();
    //     assert!(select.len() == 3);
    //     //let _ = super::DiseasesTable::delete(&d).await;
    //     //assert!(super::DiseasesTable::select(&selector_1).await.unwrap().len() == 0);
    // }

}



