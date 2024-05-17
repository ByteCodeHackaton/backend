## users (пользователи нуждающиеся в помощи)
|id|fio|path_from|path_to|request_date|average_path_time|note|place|is_confirmed|
|--|--|--|--|--|--|--|--|--|
| 00ccebbc-13e0-7000-8b18-6150ad2d0c05 |Ифванов Иван Иванович|1 | 9 | 2024-01-01 | 00:12:59 | заметка Раскажите о себе ....| (место вчтречи) У входных дверей/у турникетов/ | 0 |

### создание таблицы sql
```sql
CREATE TABLE IF NOT EXISTS  employees (
            id TEXT PRIMARY KEY NOT NULL, 
            fio TEXT NOT NULL, 
            path_from INTEGER NOT NULL, 
            path_to INTEGER NOT NULL, 
            request_date TEXT NOT NULL, 
            average_path_time TEXT NOT NULL, 
            note TEXT, 
            palce TEXT NOT NULL, 
            is_confirmed INTEGER NOT NULL DEFAULT 0
            );
```

