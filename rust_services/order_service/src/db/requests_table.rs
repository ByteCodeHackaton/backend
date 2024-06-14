use std::{borrow::Cow, ops::Deref};

use logger::backtrace;
use serde_json::json;
use db::{anyhow, get_connection, SqliteRow, Execute, FromRow, Id, IdSelector, Operations, QuerySelector, Row, Selector, SortingOrder};
use utilites::Date;
use uuid::Uuid;

use super::DB_DATE_FORMAT;

#[derive(Debug)]
///Запрос на заявку
pub struct RequestsTable
{
    /// Уникальный идентификатор заявки
    id: String,
    /// Время регистрации заявки
    date: Date,
    /// Уникальный идентификатор пассажира
    passagier_id: String,
    /// Категория пассажира
    passagier_category: String,
    /// Время встречи с пассажиром и начало его сопровождения
    request_start_date: Date,
    /// с какой станции забрать пассажира (id)
    path_from_id: String,
    /// на какую станцию перевезти пассажира (id)
    path_to_id: String,
    ///среднее время между станцииями в минутах
    average_path_time: u32,
    /// заметка оставленная пассажиром
    note: Option<String>,
    /// место встречи пассажира
    place: String,
    /// Количество сотрудников мужчин выделяемых на данную заявку
    insp_male_count: u32,
    /// Количество сотрудников женщин выделяемых на данную заявку
    insp_female_count: u32,
}


impl<'a> Id<'a> for RequestsTable
{
    fn get_id(&'a self)-> Uuid
    {
        Uuid::parse_str(&self.id).unwrap()
    }
}

impl FromRow<'_, SqliteRow> for RequestsTable
{
    fn from_row(row: &SqliteRow) -> db::Result<Self> 
    {
        let date: String = row.try_get("date")?;
        let date = Date::parse(date).unwrap();
        let request_start_date: String = row.try_get("request_start_date")?;
        let request_start_date = Date::parse(request_start_date).unwrap();
        Ok(Self
        {
            id: row.try_get("id")?,
            date,
            passagier_id: row.try_get("passagier_id")?,
            passagier_category: row.try_get("passagier_category")?,
            request_start_date,
            path_from_id: row.try_get("path_from_id")?,
            path_to_id: row.try_get("path_to_id")?,
            average_path_time: row.try_get("average_path_time")?,
            note: row.try_get("note")?,
            place: row.try_get("place")?,
            insp_male_count: row.try_get("insp_male_count")?,
            insp_female_count: row.try_get("insp_female_count")?,
        })
    }
}

impl<'a> Operations<'a> for RequestsTable
{
    fn table_name() -> &'static str 
    {
       "requests"
    }
    fn create_table() -> String 
    {  
        ["CREATE TABLE IF NOT EXISTS ", Self::table_name(), " (
            id TEXT PRIMARY KEY NOT NULL,
            date TEXT NOT NULL,
            passagier_id TEXT NOT NULL, 
            passagier_category TEXT NOT NULL, 
            request_start_date TEXT NOT NULL, 
            path_from_id TEXT NOT NULL,
            path_to_id TEXT NOT NULL,
            average_path_time INTEGER NOT NULL,
            note TEXT,
            place TEXT NOT NULL,
            insp_male_count INTEGER NOT NULL DEFAULT 1,
            insp_female_count INTEGER NOT NULL DEFAULT 0
            );"].concat()
    }
    fn full_select() -> String 
    {
        ["SELECT 
        id,
        date,
        passagier_id, 
        passagier_category, 
        request_start_date,
        path_from_id,
        path_to_id,
        average_path_time,
        note,
        place,
        insp_male_count,
        insp_female_count
        FROM ", Self::table_name()].concat()
    }
    async fn update(&'a self) -> anyhow::Result<()>
    {
        let mut c = get_connection().await?;
        let sql = ["UPDATE ", Self::table_name(),
        " SET 
        date = $2
        passagier_id = $3
        passagier_category = $4,
        request_start_date = $5,
        path_from_id = $6,
        path_to_id = $7,
        average_path_time = $8,
        note = $9,
        place = $10,
        insp_male_count = $11,
        insp_female_count = $12
        WHERE id = $1"].concat();
        db::query(&sql)
        .bind(self.id.to_string())
        .bind(&self.date.format(DB_DATE_FORMAT))
        .bind(&self.passagier_id)
        .bind(&self.passagier_category)
        .bind(&self.request_start_date.format(DB_DATE_FORMAT))
        .bind(&self.path_from_id)
        .bind(&self.path_to_id)
        .bind(&self.average_path_time)
        .bind(&self.note)
        .bind(&self.place)
        .bind(&self.insp_male_count)
        .bind(&self.insp_female_count)
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
        date,
        passagier_id, 
        passagier_category, 
        request_start_date,
        path_from_id,
        path_to_id,
        average_path_time,
        note,
        place,
        insp_male_count,
        insp_female_count) 
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)"].concat();
        db::query(&sql)
        .bind(self.id.to_string())
        .bind(&self.date.format(DB_DATE_FORMAT))
        .bind(&self.passagier_id)
        .bind(&self.passagier_category)
        .bind(&self.request_start_date.format(DB_DATE_FORMAT))
        .bind(&self.path_from_id)
        .bind(&self.path_to_id)
        .bind(&self.average_path_time)
        .bind(&self.note)
        .bind(&self.place)
        .bind(&self.insp_male_count)
        .bind(&self.insp_female_count)
        .execute(&mut c).await?;
        Ok(())
    }
    async fn add_or_ignore(&'a self) -> anyhow::Result<()>
    {
        let mut c = get_connection().await?;
        let sql = ["INSERT OR IGNORE INTO ", Self::table_name(), 
        " (
       id,
        date,
        passagier_id, 
        passagier_category, 
        request_start_date,
        path_from_id,
        path_to_id,
        average_path_time,
        note,
        place,
        insp_male_count,
        insp_female_count) 
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)"].concat();
        db::query(&sql)
        .bind(self.id.to_string())
        .bind(&self.date.format(DB_DATE_FORMAT))
        .bind(&self.passagier_id)
        .bind(&self.passagier_category)
        .bind(&self.request_start_date.format(DB_DATE_FORMAT))
        .bind(&self.path_from_id)
        .bind(&self.path_to_id)
        .bind(&self.average_path_time)
        .bind(&self.note)
        .bind(&self.place)
        .bind(&self.insp_male_count)
        .bind(&self.insp_female_count)
        .execute(&mut c).await?;
        Ok(())
    }
}

impl RequestsTable
{

    ///`rows` - количество записей получаемых из базы данных<br>
    /// `offset` - с какой позиции начинать
    pub async fn get_with_offset(rows: u32, offset: u32, params: Option<Vec<(&str, &str)>>) -> anyhow::Result<Vec<Self>> 
    {
        let ids_offset_selector = Selector::new_concat(&["SELECT id FROM ", Self::table_name()])
        .add_params(params)
        .sort(SortingOrder::Asc("date"))
        .limit(&rows)
        .offset(&offset);
        let users_ids: Vec<IdSelector> = Self::select_special_type(&ids_offset_selector).await?;
        let id_in = users_ids.into_iter().map(|m| m.0).collect::<Vec<String>>();
        let selector = Selector::new(&Self::full_select())
        .where_in(&id_in)
        .sort(SortingOrder::Asc("date"));
        let packets = Self::select(&selector).await?;
        Ok(packets)
    }
}

#[cfg(test)]
mod tests
{
    use db::{Operations, QuerySelector, Selector};
    use logger::StructLogger;

    use super::RequestsTable;


    // use super::{Operations, Selector, QuerySelector};
    #[tokio::test]
    async fn test_add_user()
    {
        StructLogger::initialize_logger();
        super::super::initialize_db().await;
        let id = "d428fc2b-db42-4737-a211-414ffc41809d".to_string();
        let p_id = "fa77873a-92f7-42d1-9a19-a79e862b3fc1".to_owned();
        let user = RequestsTable
        {
            id: id.clone(),
            date: utilites::Date::now(),
            passagier_id: p_id,
            passagier_category: "УБ".to_owned(),
            request_start_date: utilites::Date::new_date_time(14, 6, 2024, 18, 40, 23),
            path_from_id: "NB2383438".to_owned(),
            path_to_id: "NB945894".to_owned(),
            average_path_time: 33,
            note: None,
            place: "Вестибюль".to_owned(),
            insp_male_count: 1,
            insp_female_count: 1
        };
        let re = RequestsTable::add_or_replace(&user).await;
        logger::info!("{:?}", re);
        let station_id = "NB945894".to_owned();
        let selector_1 = Selector::new(&RequestsTable::full_select())
        .add_param("path_to_id", &station_id);
        println!("{}", selector_1.query().0);
        let select = RequestsTable::select(&selector_1).await.unwrap();
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



