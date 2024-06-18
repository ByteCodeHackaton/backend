// use std::{borrow::Cow, ops::Deref};
// use logger::backtrace;
// use serde_json::json;
// use db::{anyhow, get_connection, SqliteRow, Execute, FromRow, Id, IdSelector, Operations, QuerySelector, Row, Selector, SortingOrder};
// use utilites::Date;
// use uuid::Uuid;
// use super::DB_DATE_FORMAT;


// type Employee struct {
// 	Date           string `json:"date,omitempty"`
// 	Timework       string `json:"time_work,omitempty"`
// 	Id             string `json:"id,omitempty"` // Уникальный идентификатор сотрудника
// 	Fio            string `json:"fio,omitempty"`
// 	Uchastok       string `json:"uchastok,omitempty"`
// 	Smena          string `json:"smena,omitempty"`
// 	Rank           string `json:"rank,omitempty"`
// 	Sex            string `json:"sex,omitempty"`
// 	Phone_work     string `json:"phone_work,omitempty"`
// 	Phone_personal string `json:"phone_personal,omitempty"`
// 	Tab_number     string `json:"tab_number,omitempty"`
// 	Type_work      string `json:"type_work,omitempty"`
// 	Id_role        string `json:"id_role,omitempty"`
// }


// #[derive(Debug)]
// ///Заявка
// pub struct EmployeesTable
// {
//     /// Уникальный идентификатор
//     id: String,
//     ///08:00-18:00
//     time_work: String,
//     fio: String,
//     ///должность
//     rank: String,
//     sex: String,
//     phone_work: String,
//     phone_personal: String,
//     id_role: String,
//     work_type: String
// }


// impl<'a> Id<'a> for EmployeesTable
// {
//     fn get_id(&'a self)-> Uuid
//     {
//         Uuid::parse_str(&self.id).unwrap()
//     }
// }

// impl FromRow<'_, SqliteRow> for EmployeesTable
// {
//     fn from_row(row: &SqliteRow) -> db::Result<Self> 
//     {
//         Ok(Self
//         {
//             id: row.try_get("id")?,
//             time_work: row.try_get("time_work")?,
//             fio: row.try_get("fio")?,
//             rank: row.try_get("rank")?,
//             sex: row.try_get("sex")?,
//             phone_work: row.try_get("phone_work")?,
//             phone_personal: row.try_get("phone_personal")?,
//             start_work,
//             end_work
//         })
//     }
// }

// impl<'a> Operations<'a> for EmployeesTable
// {
//     fn table_name() -> &'static str 
//     {
//        "orders"
//     }
//     fn create_table() -> String 
//     {  
//         ["CREATE TABLE IF NOT EXISTS ", Self::table_name(), " (
//             id TEXT PRIMARY KEY NOT NULL,
//             order_id TEXT NOT NULL,
//             employee_id TEXT NOT NULL, 
//             start_work TEXT NOT NULL, 
//             end_work TEXT NOT NULL
//             );"].concat()
//     }
//     fn full_select() -> String 
//     {
//         ["SELECT 
//         id,
//         order_id,
//         employee_id, 
//         start_work, 
//         end_work
//         FROM ", Self::table_name()].concat()
//     }
//     async fn update(&'a self) -> anyhow::Result<()>
//     {
//         let mut c = get_connection().await?;
//         let sql = ["UPDATE ", Self::table_name(),
//         " SET 
//         order_id = $2
//         employee_id = $3
//         start_work = $4,
//         end_work = $5,
//         WHERE id = $1"].concat();
//         db::query(&sql)
//         .bind(&self.id)
//         .bind(&self.order_id)
//         .bind(&self.employee_id)
//         .bind(&self.start_work.format(DB_DATE_FORMAT))
//         .bind(&self.end_work.format(DB_DATE_FORMAT))
//         .execute(&mut c).await?;
//         Ok(())
//     }
//    async fn select<Q: QuerySelector<'a>>(selector: &Q) -> db::anyhow::Result<Vec<Self>> 
//    {
//         let mut c = get_connection().await?;
//         let query = selector.query();
//         let mut res = db::query_as::<_, Self>(&query.0);
//         if let Some(params) = query.1
//         {
//             for p in params
//             {
//                 res = res.bind(p);
//             }
//         };
//         let r = res.fetch_all(&mut c)
//         .await?;
//         Ok(r)
//    }

//     async fn add_or_replace(&'a self) -> db::anyhow::Result<()>
//     {
//         let mut c = get_connection().await?;
//         let sql = ["INSERT OR REPLACE INTO ", Self::table_name(), 
//         " (
//          id,
//         order_id,
//         employee_id, 
//         start_work, 
//         end_work) 
//         VALUES ($1, $2, $3, $4, $5)"].concat();
//         db::query(&sql)
//         .bind(&self.id)
//         .bind(&self.order_id)
//         .bind(&self.employee_id)
//         .bind(&self.start_work.format(DB_DATE_FORMAT))
//         .bind(&self.end_work.format(DB_DATE_FORMAT))
//         .execute(&mut c).await?;
//         Ok(())
//     }
//     async fn add_or_ignore(&'a self) -> anyhow::Result<()>
//     {
//         let mut c = get_connection().await?;
//         let sql = ["INSERT OR IGNORE INTO ", Self::table_name(), 
//         " (
//         id,
//         order_id,
//         employee_id, 
//         start_work, 
//         end_work) 
//         VALUES ($1, $2, $3, $4, $5)"].concat();
//         db::query(&sql)
//         .bind(&self.id)
//         .bind(&self.order_id)
//         .bind(&self.employee_id)
//         .bind(&self.start_work.format(DB_DATE_FORMAT))
//         .bind(&self.end_work.format(DB_DATE_FORMAT))
//         .execute(&mut c).await?;
//         Ok(())
//     }
// }

// impl EmployeesTable
// {
   
// }

// #[cfg(test)]
// mod tests
// {
//     use db::{Operations, QuerySelector, Selector};
//     use logger::StructLogger;
//     use crate::db::orders_table::OrdersTable;
    
//     #[tokio::test]
//     async fn test_add_order()
//     {
//         StructLogger::initialize_logger();
//         super::super::initialize_db().await;
//         let id = "d428fc2b-db42-4737-a211-414ffc41809d".to_string();
//         let o_id = "fa77873a-92f7-42d1-9a19-a79e862b3fc1".to_owned();
//         let e_id = "fa77873a-ffff-42d1-9a19-a79e862b3fc1".to_owned();
//         let user = OrdersTable
//         {
//             id: id.clone(),
//             order_id: o_id.clone(),
//             employee_id: e_id.clone(),
//             start_work: utilites::Date::new_date_time(14, 6, 2024, 18, 40, 23),
//             end_work: utilites::Date::new_date_time(14, 6, 2024, 19, 28, 0),
//         };
//         let re = OrdersTable::add_or_replace(&user).await;
//         logger::info!("{:?}", re);
//         let station_id = "NB945894".to_owned();
//         let selector_1 = Selector::new(&OrdersTable::full_select());
//         println!("{}", selector_1.query().0);
//         let select = OrdersTable::select(&selector_1).await.unwrap();
//         println!("{:?}", &select);
//         assert!(select.len() == 1);
//     }
// }



