use std::sync::{Arc, Mutex};

use logger::{debug, error, info};
use utilites::Date;
use uuid::Timestamp;

use crate::{employees::{self, AvalibleEmployees, Employees, ALL}, error::OrderError, order::{ Order, RequestOrder, ORDERS}};


pub fn add_test_workers()
{
    let emp: Vec<Employees> = vec![
        Employees::new("Карасев Артём Ильич"),
        Employees::new("Кузнецов Иван Ильич"),
        Employees::new("Калачев Михаил Романович"),
        Employees::new("Латышева Нина Григорьевна"),
        Employees::new("Орлов Павел Тимофеевич"),
        Employees::new("Голованова Дарина Львовна"),
        Employees::new("Соколова Варвара Денисовна"),
        Employees::new("Аникина София Петровна"),
        Employees::new("Гусева Агата Глебовна"),
        Employees::new("Мельников Виктор Егорович"),
        Employees::new("Полякова Вера Святославовна"),
        Employees::new("Богомолов Лев Владимирович"),
        Employees::new("Иванова Ульяна Данииловна"),
        Employees::new("Еремина Виктория Леонидовна"),
        Employees::new("Комарова Мирослава Александровна"),
        Employees::new("Киселева Виктория Егоровна"),
        Employees::new("Рудакова Елизавета Родионовна"),
        Employees::new("Королева Таисия Максимовна"),
        Employees::new("Мартынов Никита Константинович"),
        Employees::new("Сахаров Захар Андреевич"),
    ];
    ALL.set(Arc::new(Mutex::new(emp.clone())));

    let ava: Vec<AvalibleEmployees> = vec![
        //войковская
        AvalibleEmployees::new(&emp[3], Date::new_date(2, 6, 2024), "nd77715428"),
        //марксистская
        AvalibleEmployees::new(&emp[7], Date::new_date(2, 6, 2024), "nd86121438"),
        //тверская
        AvalibleEmployees::new(&emp[2], Date::new_date(2, 6, 2024), "nd52567902")
       
    ];
    crate::employees::FREE.set(Arc::new(Mutex::new(ava)));

}

//TODO пока непонятно что делать если нужно несколько сотрудников на одного человека
//сейчас мы выбираем первого ближайшего свободного сотрудника
pub async fn add_order(ord: RequestOrder) -> Result<Order, OrderError>
{
    if let Some(avalible) = employees::search_by_station(&ord.path_from, &ord.request_date)
    {
        info!("Для заявки {}->{} есть сотрудник находящийся на {}", &ord.path_from, &ord.path_to, &avalible.station_id);
        let order = search_in_orders(&ord, &avalible, None).await;
        if order.is_ok()
        {
            return order;
        }
    }
    else 
    {
        let stations = find_nearest_stations(&ord.path_from).await?;
        for s in stations
        {
            if let Some(avalible) = employees::search_by_station(&s.0, &ord.request_date)
            {
                info!("Для заявки {}->{} подобран сотрудник находящийся в пределах 10 минут, на {}", &ord.path_from, &ord.path_to, &avalible.station_id);
                let order = search_in_orders(&ord, &avalible, Some((&ord.path_from, &s.0))).await;
                if order.is_ok()
                {
                    return order;
                }
            }
        }
    }
    return Err(OrderError::NotFreeWorkers("По текущим параметрам заявки нет возможности поставить в работу сотрудника (или на доступных станциях на дату заявки никто не дежурит, либо сотрудники находятся дальше чем в 10 минутах езды от станции указанной в заявке)".to_owned()));
}


async fn search_in_orders(ord: &RequestOrder,  avalible: &AvalibleEmployees, correction: Option<(&str, &str)>) -> Result<Order, OrderError>
{
    //если работник с другой станции прибавляем к началу временного отрезка время чтобы добраться до целевой станции
    let worker_can_start_from = match correction
    {
        Some(c) => 
        {
            find_path(c.0, c.1).await?
        },
        None => 0
    };
    let o1_time = ord.request_date.clone().sub_minutes(worker_can_start_from as i64);
    let minutes = find_path(&ord.path_from, &ord.path_to).await?;
    let o2_time = ord.request_date.clone().add_minutes(minutes as i64);
    let orders_with_worker = super::order::get_orders(avalible);
    //значит данный работник не занят можно его брать
    if orders_with_worker.is_empty()
    {
        let new_order = Order
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
            employess: vec![avalible.id.clone()]
        };
        return Ok(new_order);
    }
    else 
    {
        for o in orders_with_worker
        {
            let timeline = vec![o.busy_time_range()];
            let cmp = Date::in_range((&o1_time, &o2_time), &timeline);
            //у работника данный таймлайн свободен можно его брать
            if cmp.is_none()
            {
                let new_order = Order
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
                    employess: vec![avalible.id.clone()]
                };
                return Ok(new_order);
            }
        }    
    }
    return Err(OrderError::NotFreeWorkers("нет свободных сотрудников".to_owned()));
}

pub async fn find_nearest_stations(id: &str) -> Result<Vec<(String, usize)>, super::error::OrderError>
{
    let path = format!("http://localhost:8888/nearest?id={}&time={}", id, 10);
    let resp = reqwest::get(path).await?;
    let json: serde_json::Value = resp.json().await?;
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
    let resp = reqwest::get(path).await?;
    let json: serde_json::Value = resp.json().await?;
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
    use logger::debug;
    use utilites::Date;

    use crate::order::RequestOrder;

    #[tokio::test]
   async fn test_nearest_station()
   {
        logger::StructLogger::initialize_logger();
        super::add_test_workers();
        let req1 = RequestOrder::new("Заматова Мамата Ватовна", "nd83680109", "nd68989070", Date::new_date_time(2, 6, 2024, 9, 30, 0), None, crate::order::Place::OnCenter);
        let o = super::add_order(req1).await;
        debug!("{:?}", o);
   }

   #[tokio::test]
   async fn test_point_station()
   {
        logger::StructLogger::initialize_logger();
        super::add_test_workers();
        let req1 = RequestOrder::new("Заматова Мамата Ватовна", "nd52567902", "nd77715428", Date::new_date_time(2, 6, 2024, 9, 30, 0), None, crate::order::Place::OnCenter);
        let o = super::add_order(req1).await;
        debug!("{:?}", o);
        
   }
}