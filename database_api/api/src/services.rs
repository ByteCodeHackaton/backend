// pub async fn get_senders() -> Result<Json<Value>, AppError>
    // {
    //     let q = AddresseTable::query(None, []);
    //     return Ok(Json(json!(crate::models::Response::new(q.unwrap()))));
    // }
    ///http://127.0.0.1:3000/packets?source_id=6d8c1ef5-a5ea-4dd9-a97d-5ee80f0663b1&date1=2023-02-14T13:38:33&date2=2023-02-14T13:38:40
    pub async fn get_packets(Query(params): Query<PacketParams>) -> Result<Json<crate::models::Response::<Vec<Box<PacketInfo>>>>, AppError>
    {
        let mut selector: Vec<String> = vec![];
        if let Some(d1) = params.date1.as_ref()
        {
            let s = ["datetime(delivery_time) > ", "datetime('", d1, "')"].concat();
            selector.push(s)
        }
        if let Some(d2) = params.date2.as_ref()
        {
            let s = ["datetime(delivery_time) < ", "datetime('", d2, "')"].concat();
            selector.push(s)
        }
        if let Some(id) = params.source_id.as_ref()
        {
            let s = ["sender_info->'sourceGuid' = '\"", id, "\"'"].concat();
            selector.push(s)
        }
        if let Some(updated) = params.updated.as_ref()
        {
            let update_date = medo_settings::convert_system_time_offset(SystemTime::now(), *updated as u64);
            let s = ["update_key' >= '\"", &update_date.unwrap(), "\"'"].concat();
            selector.push(s)
        }
        let selector = selector.join(" AND ");
        let mut select = Some(selector.as_ref());
        if selector.len() == 0
        {
            select = None;
        }
        let res = PacketInfo::query(select, []);
        if res.is_err()
        {
            let e =  res.err().as_ref().unwrap().to_string();
            error!("{}", &e);
            //Выводим общее сообщение об огшибке, иначе в нее попадет вся информауия о структуре SQL
            return Err(AppError::SqlError);
        }
        return Ok(Json(crate::models::Response::new(res.unwrap())));
    }


    
    pub async fn set_visibility(Path(id): Path<String>) -> impl IntoResponse
    {
        let res =  PacketInfo::query(Some("header_id = :header_id"), &[(":header_id", &id)]);
        if res.is_err()
        {
            let e =  res.err().as_ref().unwrap().to_string();
            error!("{}", &e);
            return Err(AppError::SqlError);
        }
        let exists = res.unwrap();
        if exists.is_empty()
        {
            let e =  ["Ошибка установки видимости для пакета ", &id, " такой идентификатор отсутсвует в базе данных"].concat();
            error!("{}", &e);
            //Выводим общее сообщение об огшибке, иначе в нее попадет вся информауия о структуре SQL
            return Err(AppError::CustomError(e));
        }
        let mut packet = exists.iter().nth(0).as_mut().unwrap().clone();
        packet.visible = !packet.visible;
        let del_operation = packet.update();
        if del_operation.is_err()
        {
            let e =  del_operation.err().as_ref().unwrap().to_string();
            error!("{}", &e);
            return Err(AppError::CustomError(e));
        }
        else
        {
            return Ok(Json(json!(crate::models::Response::new(true))));
        }
    }
