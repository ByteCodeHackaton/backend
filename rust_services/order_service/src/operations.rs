use std::sync::{Arc};
use serde_json::Value;
use tokio::sync::Mutex;

use logger::{debug, error, info};
use utilites::Date;
use uuid::Timestamp;

use crate::{employees::{self, AvalibleEmployees, Employees, ALL}, error::OrderError, order::{ Order, RequestOrder, ORDERS}};

//сейчас мы выбираем первого ближайшего свободного сотрудника
pub async fn add_order(ord: &RequestOrder) -> Result<Order, OrderError>
{
    let avalible = search_avalible_employees(ord).await?;
    let order = search_in_orders(&ord, avalible).await;
    if order.is_ok()
    {
        return order;
    }
    return Err(OrderError::NotFreeWorkers("По текущим параметрам заявки нет возможности поставить в работу сотрудника (или на доступных станциях на дату заявки никто не дежурит, либо сотрудники находятся дальше чем в 10 минутах езды от станции указанной в заявке)".to_owned()));
}


pub async fn search_avalible_employees(ord: &RequestOrder) -> Result<Vec<(AvalibleEmployees, Option<(String, String)>)>, OrderError>
{
    let aval_all = employees::get_workers_on_date(&ord.request_date).await;
    let mut avalible: Vec<(AvalibleEmployees, Option<(String, String)>)> = aval_all.iter().filter(|s| &s.station_id == &ord.path_from)
    .map(|m| (m.clone(), None)).collect();
    if !avalible.is_empty()
    {
        info!("Для заявки {}->{} есть {} сотрудников находящихся на {}", &ord.path_from, &ord.path_to, avalible.len(), &ord.path_from);
        if (ord.employees_count as usize) <= avalible.len()
        {
            return Ok(avalible);
        }
    }
    let stations = find_nearest_stations(&ord.path_from).await?;
    for s in stations
    {
        let av: Vec<(AvalibleEmployees, Option<(String, String)>)> = aval_all
                                                                        .iter()
                                                                        .filter(|e| &e.station_id == &s.0)
                                                                        .map(|m| (m.clone(), Some((s.0.clone(), ord.path_from.clone()))))
                                                                        .collect();
        if !av.is_empty()
        {
            for a in av
            {
                logger::info!("на удалении {} мин, на станции {} найден сотрудник {} ",s.1, s.0, &a.0.employee_id);
                if avalible.iter().all(|all| &a.0.employee_id != &all.0.employee_id)
                {
                    info!("Для заявки {}->{} подобран сотрудник находящийся в пределах {} минут, на станции {}", &ord.path_from, &ord.path_to, s.1, &s.0);
                    avalible.push(a);
                    if (ord.employees_count as usize) <= avalible.len()
                    {
                        return Ok(avalible);
                    }
                }
            }
        }
    }
    if !avalible.is_empty()
    {
        logger::warn!("Внимание, для заявки {} выделено {} доступных сотрудников из {} запрошенных сотрудников", &ord.id, avalible.len(), ord.employees_count);
        return Ok(avalible);
    }
    return Err(OrderError::NotFreeWorkers("По текущим параметрам заявки нет возможности поставить в работу сотрудника (или на доступных станциях на дату заявки никто не дежурит, либо сотрудники находятся дальше чем в 60 минутах езды от станции указанной в заявке)".to_owned()));
}

//сюда передаем только нужное количество
async fn search_in_orders(ord: &RequestOrder, avalible: Vec<(AvalibleEmployees, Option<(String, String)>)>) -> Result<Order, OrderError>
{
    //если работник с другой станции прибавляем к началу временного отрезка время чтобы добраться до целевой станции
    let mut worker_can_start_from = 0;
    for correction in &avalible
    {
        if let Some(c) = correction.1.as_ref()
        {
            let p = find_path(&c.0, &c.1).await?;
            if worker_can_start_from< p
            {
                worker_can_start_from = p;
            }
        }
    }
    //от начального времени убираем время которое необходимо сотруднику чтобы добраться до целевой станции, если он уже не на ней
    let o1_time = ord.request_date.clone().sub_minutes(worker_can_start_from as i64);
    let minutes = find_path(&ord.path_from, &ord.path_to).await?;
    //конечное время заявки состоит из времени на поездку
    let o2_time = ord.request_date.clone().add_minutes(minutes as i64);
    let mut new_order = Order
    {
        id: ord.id.clone(),
        fio: ord.fio.clone(),
        request_date: ord.request_date.clone(),
        path_from: ord.path_from.clone(),
        path_to: ord.path_to.clone(),
        average_path_time: minutes,
        note: ord.note.clone(),
        place: ord.place.clone(),
        start_work: o1_time.clone(),
        end_work: o2_time.clone(),
        employess: vec![]
    };
    //лочим ордера до момента когда можно будет выбрать временные окна и добавить сотрудников
    let g = ORDERS.get_or_init(|| Arc::new(Mutex::new(vec![])));
    let mut guard = g.lock().await;
    for a in &avalible
    {

        let orders_with_worker: Vec<&Order> = guard.iter()
        .filter(|f| f.employess.iter()
            .find(|e| *e == &a.0.id).is_some()).collect();
        //значит данный работник не занят можно его брать
        if orders_with_worker.is_empty()
        {
            new_order.employess.push(a.0.employee_id.clone());
        }
        else 
        {
            //проверяем окна свободного времени у данного работника, если находим такое, то создаем заявку
            for o in orders_with_worker
            {
                let timeline = vec![o.busy_time_range()];
                let cmp = Date::in_range((&o1_time, &o2_time), &timeline);
                //у работника данный таймлайн свободен можно его брать
                if cmp.is_none()
                {
                    new_order.employess.push(a.0.employee_id.clone());
                }
            }    
        }
        //если набралось количество необходимое в заявке, то харош
        if new_order.employess.len() == ord.employees_count as usize
        {
            break;
        }
    }
    if new_order.employess.len() > 0
    {
        if let Some(index) = guard.iter().position(|f| &f.id == &new_order.id)
        {
            guard[index] = new_order.clone();
        }
        else
        {
            guard.push(new_order.clone());
        }

        return Ok(new_order);
    }
    else 
    {
        return Err(OrderError::NotFreeWorkers("нет свободных сотрудников".to_owned()));
    }
}

pub async fn find_nearest_stations(id: &str) -> Result<Vec<(String, usize)>, super::error::OrderError>
{
    let path = format!("http://localhost:8888/nearest?id={}&time={}", id, 60);
    let json = utilites::http::get::<Value>(path.parse().unwrap()).await?;
    //let resp = reqwest::get(path).await?;
    //let json: serde_json::Value = resp.json().await?;
    if json["success"].as_bool().unwrap() == false
    {
        let e = json["message"].as_str().unwrap();
        error!("{}", e);
        return Err(OrderError::StationServiceError(e.to_owned()));
    }
    else
    {
        debug!("{}", json["responseObject"]);
        let arr: Vec<(String, usize)> = json["responseObject"].as_array().unwrap().iter().map(|m| (m["station"]["node_id"].as_str().unwrap().to_owned(), m["time"].as_u64().unwrap() as usize)).collect();
        return Ok(arr);
    } 
}
pub async fn find_path(from: &str, to: &str) -> Result<u32, super::error::OrderError>
{
    let path = format!("http://localhost:8888/path?from={}&to={}", from, to);
    let json = utilites::http::get::<Value>(path.parse().unwrap()).await?;
    //let resp = reqwest::get(path).await?;
    //let json: serde_json::Value = resp.json().await?;
    if json["success"].as_bool().unwrap() == false
    {
        let e = json["message"].as_str().unwrap();
        error!("{}", e);
        return Err(OrderError::StationServiceError(e.to_owned()));
    }
    else
    {
        let time = json["responseObject"]["time"].as_u64().unwrap();
        return Ok(time as u32);
    } 
}


#[cfg(test)]
mod tests
{
    use hyper::Uri;
    use logger::debug;
    use serde_json::Value;
    use utilites::Date;

    use crate::{api, order::RequestOrder, Workday};


   #[tokio::test]
   async fn test_get_workers()
   {
        logger::StructLogger::initialize_logger();
        let uri: Uri = "http://localhost:5010/api/v1/workday/date/list?limit=1000&date=2024-06-19T00:00:00".parse().unwrap();
        let result = crate::http::get::<Value>(uri).await.unwrap();
        let arr = result["document"]["details"].as_array().unwrap();
        for wd in arr 
        {
            logger::info!("{:?}", &wd["employee_id"].as_str().unwrap());
            let wd = serde_json::from_value::<Workday>(wd.to_owned()).unwrap();
        }
        
        
        
   }

   #[tokio::test]
   async fn test_complex()
   {
        logger::StructLogger::initialize_logger();
        //super::add_test_workers();
        let req1 = RequestOrder::new("Иванова Ивана Ивановна", "nd86441857", "nd49069277", Date::new_date_time(19, 6, 2024, 9, 30, 0), 2, None, crate::order::Place::OnCenter);
        let o = super::add_order(&req1).await;
        debug!("{:?}", o);
        
   }
}