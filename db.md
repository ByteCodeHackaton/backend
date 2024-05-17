## employees (сотрудники)
|fio|vacations|is_busy|station_service|current_station|
|--|--|--|--|--|
|Ифванов Иван Иванович|[пример](#список-отпусков-сотрудника)| 0 | [пример](#обслуживаемые-станции-идентификаторы) | 2 или NULL |

### список отпусков сотрудника
```json
{
    [ 
        from: "2024-01-01",
        to: "2024-01-10",
        from: "2024-08-01",
        to: "2024-09-01"
    ]
}
```

### обслуживаемые станции (идентификаторы)
```json
{
    [ 
        1,
        2,
        3,
        4
    ]
}
```

### создание таблицы sql
```sql
CREATE TABLE IF NOT EXISTS  employees (
            id TEXT PRIMARY KEY NOT NULL, 
            fio TEXT NOT NULL, 
            vacations JSON DEFAULT('[]'), 
            is_busy INTEGER NOT NULL DEFAULT 0,
            station_service JSON DEFAULT('[]'), 
            current_station INTEGER
            );
```