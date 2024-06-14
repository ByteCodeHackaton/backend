use utilites::Date;

use crate::error::OrderError;

struct Workday
{
    /// Уникальный идентификатор рабочего дня сотрудника
    id: String,
    /// Уникальный идентификатор сотрудника
    employee_id: String,
    /// Дата выхода
    date_work: Date,
    /// Время работы (07:00-19:00, 08:00-20:00, 20:00-08:00, 08:00-17:00)
    time_work: Date,
    /// Статус рабочего дня
    status: String,
    /// Дополнительная смена (выход не по своему графику, дата)
    extra_shift: Option<String>,
    /// Учеба с отрывом от производства (дата от-до)                             
    education: Option<String>,
    /// Изменение времени работы (если время работы не совпадает с графиком)                                 
    custom_time: Option<String>,
    /// Стажировка (заявки только совместно с наставником)                                          
    intern: Option<String>
}

// pub async fn get_work_days() -> Result<Workday, OrderError>
// {
    
// }
