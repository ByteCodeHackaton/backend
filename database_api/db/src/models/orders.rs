// use std::{borrow::Cow, ops::Deref};

// use logger::backtrace;
// use serde_json::json;
// use sqlx::{Row, sqlite::SqliteRow, FromRow, Execute};
// use utilites::Date;
// use uuid::Uuid;
// use crate::AddresseTable;


// // type Order struct {
// // 	Id         string `json:"id,omitempty" example:"477354"`                   // Уникальный идентификатор заявки
// // 	Id_Pas     string `json:"id_pas,omitempty" example:"11058"`                // Уникальный идентификатор пассажира
// // 	DateTime   string `json:"datetime,omitempty" example:"24.04.2024 7:30:00"` // Дата и время начала заявки
// // 	Time3      string `json:"time3,omitempty" example:"07:13:52"`              // Время встречи с пассажиром и начало его сопровождения
// // 	Time4      string `json:"time4,omitempty" example:"07:51:11"`              // Время завершения сопровождения пассажира
// // 	Cat_pas    string `json:"cat_pas,omitempty" example:"ИЗТ"`                 // Категория пассажира
// // 	Status     string `json:"status,omitempty" example:"Заявка закончена"`     // Статус заявки
// // 	Tpz        string `json:"tpz,omitempty" example:"15.03.2024 22:48:43"`     // Время регистрации заявки
// // 	INSP_SEX_M string `json:"insp_sex_m,omitempty" example:"0"`                // Количество сотрудников мужчин выделяемых на данную заявку
// // 	INSP_SEX_F string `json:"insp_sex_f,omitempty" example:"1"`                // Количество сотрудников женщин выделяемых на данную заявку
// // 	TIME_OVER  string `json:"time_over,omitempty" example:"00:52:20"`          // Рассчитанное примерное время на выполнение заявки
// // 	Id_st1     string `json:"id_st1,omitempty" example:"5"`                    // ID начальной станции
// // 	Id_st2     string `json:"id_st2,omitempty" example:"97"`                   // ID конечной станции
// // }


// // #[derive(Clone, Serialize, Deserialize, Debug)]
// // pub struct RequestOrder
// // {
// //     pub id: String,
// //     pub fio: String,
// //     // from node id
// //     pub path_from: String,
// //     // to node id
// //     pub path_to: String,
// //     // date
// //     pub request_date: utilites::Date,
// //     pub note: Option<String>,
// //     //требуемое количество сотрудников (непонятно кто это будет решать)
// //     pub employees_count: u32,
// //     pub place: Place,
// // }

// use super::{connection::get_connection, operations::{to_json, CountRequest, Id, IdSelector, Operations, QuerySelector, Selector, SortingOrder}};
// #[derive(Debug)]
// ///Запрос на заявку
// pub struct RequestOrder
// {
//     /// Уникальный идентификатор заявки
//     id: String,
//     /// Время регистрации заявки
//     date: Date,
//     /// Уникальный идентификатор пассажира
//     passagier_id: String,
//     /// Категория пассажира
//     passagier_category: String,
//     /// Время встречи с пассажиром и начало его сопровождения
//     request_start_date: Date,
//     /// с какой станции забрать пассажира (id)
//     path_from_id: String,
//     /// на какую станцию перевезти пассажира (id)
//     path_to_id: String,
//     ///среднее время между станцииями в минутах
//     average_path_time: u32,
//     /// заметка оставленная пассажиром
//     note: Option<String>,
//     /// место встречи пассажира
//     place: String,
//     /// Количество сотрудников мужчин выделяемых на данную заявку
//     insp_male_count: u32,
//     /// Количество сотрудников женщин выделяемых на данную заявку
//     insp_female_count: u32,
// }


// impl<'a> Id<'a> for RequestOrder
// {
//     fn get_id(&'a self)-> Uuid
//     {
//         Uuid::parse_str(&self.id).unwrap()
//     }
// }

// impl FromRow<'_, SqliteRow> for RequestOrder
// {
//     fn from_row(row: &SqliteRow) -> sqlx::Result<Self> 
//     {
//         let date: String = row.try_get("date")?;
//         let date = Date::parse(date).unwrap();
//         let request_start_date: String = row.try_get("request_start_date")?;
//         let request_start_date = Date::parse(request_start_date).unwrap();
//         Ok(Self
//         {
//             id: row.try_get("id")?,
//             date,
//             passagier_id: row.try_get("passagier_id")?,
//             passagier_category: row.try_get("passagier_category")?,
//             request_start_date,
//             path_from_id: row.try_get("path_from_id")?,
//             path_to_id: row.try_get("path_to_id")?,
//             average_path_time: row.try_get("average_path_time")?,
//             note: row.try_get("note")?,
//             place: row.try_get("place")?,
//             insp_male_count: row.try_get("insp_male_count")?,
//             insp_female_count: row.try_get("insp_female_count")?,
//         })
//     }
// }

// impl<'a> Operations<'a> for RequestOrder
// {
//     fn table_name() -> &'static str 
//     {
//        "requests"
//     }
//     fn create_table() -> String 
//     {  
//         ["CREATE TABLE IF NOT EXISTS ", Self::table_name(), " (
//             id TEXT PRIMARY KEY NOT NULL,
//             date TEXT NOT NULL,
//             passagier_id TEXT NOT NULL, 
//             passagier_category TEXT NOT NULL, 
//             request_start_date TEXT NOT NULL, 
//             path_from_id TEXT NOT NULL,
//             path_to_id TEXT NOT NULL,
//             average_path_time INTEGER NOT NULL,
//             note TEXT,
//             place TEXT NOT NULL,
//             insp_male_count INTEGER NOT NULL DEFAULT 1,
//             insp_female_count INTEGER NOT NULL DEFAULT 0
//             );"].concat()
//     }
//     fn full_select() -> String 
//     {
//         ["SELECT 
//         id,
//         date,
//         passagier_id, 
//         passagier_category, 
//         request_start_date,
//         path_from_id,
//         path_to_id,
//         average_path_time,
//         note,
//         place,
//         insp_male_count,
//         insp_female_count
//         FROM ", Self::table_name()].concat()
//     }
//     async fn update(&'a self) -> anyhow::Result<()>
//     {
//         let mut c = get_connection().await?;
//         let sql = ["UPDATE ", Self::table_name(),
//         " SET 
//         task_name = $2
//         header_id = $3
//         directory = $4,
//         packet_type = $5,
//         delivery_time = $6,
//         error = $7,
//         default_pdf = $8,
//         files = $9,
//         requisites = $10,
//         sender_info = $11,
//         pdf_hash = $12,
//         update_key = $13,
//         acknowledgment = $14,
//         visible = $15,
//         trace_message = $16,
//         report_sended = $17
//         WHERE id = $1"].concat();
//         sqlx::query(&sql)
//         .bind(self.id.to_string())
//         .bind(&self.task_name)
//         .bind(&self.packet_info.header_guid)
//         .bind(&self.packet_info.packet_directory)
//         .bind(&self.packet_info.packet_type)
//         .bind(&self.packet_info.delivery_time)
//         .bind(&self.packet_info.error)
//         .bind(&self.packet_info.default_pdf)
//         .bind(&to_json(&self.packet_info.files))
//         .bind(&to_json(&self.packet_info.requisites))
//         .bind(&to_json(&self.packet_info.sender_info))
//         .bind(&self.packet_info.pdf_hash)
//         .bind(&self.packet_info.update_key)
//         .bind(&to_json(&self.packet_info.acknowledgment))
//         .bind(&self.packet_info.visible)
//         .bind(&self.packet_info.trace_message)
//         .bind(&self.report_sended)
//         .execute(&mut c).await?;
//         if let Ok(addreesses) = AddresseTable::try_from(&self.packet_info)
//         {
//             let _ = addreesses.add_or_replace().await;
//         }
//         Ok(())
//     }
//    async fn select<Q: QuerySelector<'a>>(selector: &Q) -> anyhow::Result<Vec<PacketsTable>> 
//    {
//         let mut c = get_connection().await?;
//         let query = selector.query();
//         let mut res = sqlx::query_as::<_, PacketsTable>(&query.0);
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

//     async fn add_or_replace(&'a self) -> anyhow::Result<()>
//     {
//         let mut c = get_connection().await?;
//         let sql = ["INSERT OR REPLACE INTO ", Self::table_name(), 
//         " (
//         id,
//         task_name,
//         header_id,
//         directory,
//         packet_type,
//         delivery_time,
//         error,
//         default_pdf,
//         files,
//         requisites,
//         sender_info,
//         pdf_hash,
//         update_key,
//         acknowledgment,
//         visible,
//         trace_message,
//         report_sended) 
//         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)"].concat();
//         sqlx::query(&sql)
//         .bind(self.id.to_string())
//         .bind(&self.task_name)
//         .bind(&self.packet_info.header_guid)
//         .bind(&self.packet_info.packet_directory)
//         .bind(&self.packet_info.packet_type)
//         .bind(&self.packet_info.delivery_time)
//         .bind(&self.packet_info.error)
//         .bind(&self.packet_info.default_pdf)
//         .bind(&to_json(&self.packet_info.files))
//         .bind(&to_json(&self.packet_info.requisites))
//         .bind(&to_json(&self.packet_info.sender_info))
//         .bind(&self.packet_info.pdf_hash)
//         .bind(&self.packet_info.update_key)
//         .bind(&to_json(&self.packet_info.acknowledgment))
//         .bind(&self.packet_info.visible)
//         .bind(&self.packet_info.trace_message)
//         .bind(&self.report_sended)
//         .execute(&mut c).await?;
//         if let Ok(addreesses) = AddresseTable::try_from(&self.packet_info)
//         {
//             let _ = addreesses.add_or_replace().await;
//         }
//         Ok(())
//     }
//     async fn add_or_ignore(&'a self) -> anyhow::Result<()>
//     {
//         let mut c = get_connection().await?;
//         let sql = ["INSERT OR IGNORE INTO ", Self::table_name(), 
//         " (
//         id,
//         task_name,
//         header_id,
//         directory,
//         packet_type,
//         delivery_time,
//         error,
//         default_pdf,
//         files,
//         requisites,
//         sender_info,
//         pdf_hash,
//         update_key,
//         acknowledgment,
//         visible,
//         trace_message,
//         report_sended) 
//         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)"].concat();
//         sqlx::query(&sql)
//         .bind(self.id.to_string())
//         .bind(&self.task_name)
//         .bind(&self.packet_info.header_guid)
//         .bind(&self.packet_info.packet_directory)
//         .bind(&self.packet_info.packet_type)
//         .bind(&self.packet_info.delivery_time)
//         .bind(&self.packet_info.error)
//         .bind(&self.packet_info.default_pdf)
//         .bind(&to_json(&self.packet_info.files))
//         .bind(&to_json(&self.packet_info.requisites))
//         .bind(&to_json(&self.packet_info.sender_info))
//         .bind(&self.packet_info.pdf_hash)
//         .bind(&self.packet_info.update_key)
//         .bind(&to_json(&self.packet_info.acknowledgment))
//         .bind(&self.packet_info.visible)
//         .bind(&self.packet_info.trace_message)
//         .bind(&self.report_sended)
//         .execute(&mut c).await?;
//         if let Ok(addreesses) = AddresseTable::try_from(&self.packet_info)
//         {
//             let _ = addreesses.add_or_replace().await;
//         }
//         Ok(())
//     }
// }

// impl PacketsTable
// {
//     pub async fn packets_count() -> anyhow::Result<u32>
//     {
//         //let q = ["SELECT COUNT(*) as count FROM ", Self::table_name()].concat();
//         let selector = Selector::new_concat(&["SELECT COUNT(*) as count FROM ", Self::table_name()]);
//         let count: CountRequest = Self::get_one(&selector).await?;
//         Ok(count.count)
//     }
//     //TODO добавить выборку по параметрам а не тупо всех подряд, будет и отсеивание по имени и еще по чему то
//     ///`rows` - количество записей получаемых из базы данных<br>
//     /// `offset` - с какой позиции начинать
//     pub async fn get_with_offset(rows: u32, offset: u32, params: Option<Vec<(&str, &str)>>) -> anyhow::Result<Vec<PacketsTable>> 
//     {
//         let ids_offset_selector = Selector::new_concat(&["SELECT id FROM ", Self::table_name()])
//         .add_params(params)
//         .sort(SortingOrder::Asc("delivery_time"))
//         .limit(&rows)
//         .offset(&offset);
//         let users_ids: Vec<IdSelector> = Self::select_special_type(&ids_offset_selector).await?;
//         let id_in = users_ids.into_iter().map(|m| m.0).collect::<Vec<String>>();
//         let selector = Selector::new(&Self::full_select())
//         .where_in(&id_in)
//         .sort(SortingOrder::Asc("delivery_time"));
//         let packets = Self::select(&selector).await?;
//         Ok(packets)
//     }
// }

// #[cfg(test)]
// mod tests
// {
//     use crate::PacketsTable;


//     // use super::{Operations, Selector, QuerySelector};
//     // #[tokio::test]
//     // async fn test_add_user()
//     // {
//     //     super::initialize().await;
//     //     let id = "d428fc2b-db42-4737-a211-414ffc41809d".to_string();
//     //     let dict_str = "fa77873a-92f7-42d1-9a19-a79e862b3fc1".to_owned();
//     //     let user = User
//     //     {
//     //         id: id.clone(),
//     //         name1: "Тест_2".into(),
//     //         name2: "Тестович_2".into(),
//     //         surname: "Тестов_2".into(),
//     //         san_ticket_number: "123321123".into(),
//     //         bornsday: "24.05.1983".into(),
//     //         post: Dictionary{id: dict_str.clone(), name: "123".into()},
//     //         department: Dictionary{id: dict_str.clone(), name: "123".into()},
//     //         rank: Dictionary{id: dict_str.clone(), name: "123".into()},
//     //         live_place: "Тестовое место жительства".into(),
//     //         phones: vec![
//     //             Phones{ phone_type: "тестовый".into(), phone_number: "32123".into(), is_main: false }
//     //         ],
//     //         tests: vec![
//     //             DiseaseTest{ is_active: true, date: Date::new(2024, 1, 1).unwrap().val }
//     //         ],
//     //         diseases: vec![],
//     //         statuses: vec![]
//     //     };
//     //     let _  = super::UsersTable::create().await;
//     //     let _ = super::UsersTable::add_or_replace(&user).await;
//     //     let selector_1 = Selector::new(&super::UsersTable::full_select())
//     //     .add_param("u.id", &id);
//     //     println!("{}", selector_1.query().0);
//     //     let select = super::UsersTable::select(&selector_1).await.unwrap();
//     //     println!("{:?}", &select);
//     //     assert!(select.len() == 1);
//     //     //let _ = super::DiseasesTable::delete(&d).await;
//     //     //assert!(super::DiseasesTable::select(&selector_1).await.unwrap().len() == 0);
//     // }
//     #[tokio::test]
//     async fn test_add_user()
//     {
//         logger::StructLogger::initialize_logger();
//         let paging : Vec<String> = PacketsTable::get_with_offset(3, 0, None).await.unwrap().into_iter().map(|m| m.packet_info.delivery_time).collect();
//         logger::debug!("{:?}", paging);
//     }

//     // #[tokio::test]
//     // async fn test_json_select()
//     // {
//     //     super::initialize().await;
//     //     let selector_1 = Selector::new(&super::UsersTable::full_select())
//     //     .add_json_param("phones->'is_main'", &false);
//     //     println!("{}", selector_1.query().0);
//     //     let select = super::UsersTable::select(&selector_1).await.unwrap();
//     //     println!("{:?}", &select);
//     //     assert!(select.len() == 1);
//     //     //let _ = super::DiseasesTable::delete(&d).await;
//     //     //assert!(super::DiseasesTable::select(&selector_1).await.unwrap().len() == 0);
//     // }

//     // #[tokio::test]
//     // async fn test_diseases_user_select()
//     // {
//     //     logger::StructLogger::initialize_logger();
//     //     let _ = super::initialize().await;
//     //     let select = UsersTable::get_current_diseases_users().await.unwrap();
//     //     assert!(select.len() == 1);
//     //     //let _ = super::DiseasesTable::delete(&d).await;
//     //     //assert!(super::DiseasesTable::select(&selector_1).await.unwrap().len() == 0);
//     // }
//     // #[tokio::test]
//     // async fn test_vacations_user_select()
//     // {
//     //     let _ = super::initialize().await;
//     //     let select = UsersTable::get_users_status().await.unwrap();
//     //     assert!(select.len() == 3);
//     //     //let _ = super::DiseasesTable::delete(&d).await;
//     //     //assert!(super::DiseasesTable::select(&selector_1).await.unwrap().len() == 0);
//     // }

// }



