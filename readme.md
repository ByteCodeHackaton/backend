# СТРУКТУРА ПРОЕКТА

- [ ] database_api -
- [ ] db_scheme - схемы БД (думаю надо будет удалить)
- [ ] dtservice1 - сервис RESTful API для взаимодействия с пользователями.
- [ ] sseservice - сервис SSE для обновления данных на frontend
- [ ] docs - документация

## database_api

## dtservice1

- файл ***readme.md*** содержит описание методов API, входных и выходных данных

- перед сборкой выполнить ***init.sh***

- ***Сборка***

***go build -o ../../build/api_service***

- в каталог ***build*** скопировать файл БД: ***metro.db*** и файл настроек gateway: ***init.json***

## docs
