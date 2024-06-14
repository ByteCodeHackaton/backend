|id|station_name|line|adm_area|district|mcd_station|aeroexpress_station|railway_station|railway_terminal|objec_status|global_id|
|--|--|--|--|--|--|--|--|--|--|--|
|1|Красные ворота|Сокольническая|хз|хз|||||действует|7364727|

стандартный объект у всех json полей:
```json
{
   global_id: 84738848,  
   value: "название"
}
```

```sql
CREATE TABLE "stations" (
  "id"  INTEGER NOT NULL PRIMARY KEY,
  "station_name"  TEXT NOT NULL,
  "line"  TEXT NOT NULL,
  "adm_area"  TEXT NOT NULL,
  "district"  NUMERIC NOT NULL,
  "mcd_station"  JSON DEFAULT('[]'),
  "aeroexpress_station"  JSON DEFAULT('[]'),
  "railway_station"  JSON DEFAULT('[]'),
  "railway_terminal" JSON DEFAULT('[]'),
  "objec_status"  TEXT,
  "global_id"  INTEGER
);
```

```sql
SELECT json_extract(value, '$.value')) mcd_name,
FROM stations s, json_each(json_extract(s.mcd_station, '$'))
GROUP BY mcd_name
```