# DepTrans API Service

## Назначение

- API функционал сервера, работа с данными сотрудников, пассажиров.

## Api

- [ ] [metod post /employee/set](Метод регистрации нового сотрудника метрополитена)
- [ ] [metod get  /employee/set](Метод получения списка всех сотрудников, работающих по заявкам)
- [ ] [metod get  /passengers/update](Метод регистрации нового пассажира)

## Зависимости

You will need to get the following packages to make it work:

```go
go get -u github.com/lib/pq
go get -u github.com/gorilla/mux
go get -u modernc.org/sqlite
```

## Методы

## /employee/set

на входе:

```json
{
    "date": "25.04.2024",
    "time_work": "20:00-08:00",
    "id": "1300",
    "fio": "Тестов В.И.",
    "uchastok": "ЦУ-4 (Н)",
    "smena": "2Н",
    "rank": "ЦИ",
    "sex": "Мужской"
}
```

на выходе при успешном выполнении статус 200 и:

```json
{
    "document": {
        "message": "Добавлен сотрудник: Тестовый В.И."
    }
}
```

при повторном добавлении сотрудника статус 417 и ошибка:

```txt
Ошибка добавления сотрудника: constraint failed: UNIQUE constraint failed: employees.id (1555)
```

--------------------------------------------

## /employee/list

входных параметров нет

на выходе при успешном выполнении статус 200 и:

```json
{
    "document": {
        "details": [
            {
                "date": "24.04.2024",
                "time_work": "08:00-20:00",
                "id": "3",
                "fio": "Белоусова Е,В.",
                "uchastok": "ЦУ-3",
                "smena": "1",
                "rank": "ЦИ",
                "sex": "Женский"
            },
            {
                "date": "24.04.2024",
                "time_work": "07:00-19:00",
                "id": "4",
                "fio": "Жукова Г.Б.",
                "uchastok": "ЦУ-1",
                "smena": "1",
                "rank": "ЦСИ",
                "sex": "Женский"
            }
        ]
    }
}
```

--------------------------------------------
