use std::collections::HashMap;

use logger::{error, info};
use petgraph::algo::{dijkstra, astar};
use crate::{MetroPath, Nearest, Station, GRAPH};


pub fn find_path(from_node_id: &str, to_node_id: &str) -> Result<MetroPath, String>
{
    if let Some(graph) = GRAPH.as_ref()
    {
        let i1 = graph.get_by_index(from_node_id);
        let i2 = graph.get_by_index(to_node_id);
        if i1.is_none()
        {
            let e = format!("Нода с id {} не найдена в сервисе", from_node_id);
            return Err(e);
        }
        else if i2.is_none()
        {
            let e = format!("Нода с id {} не найдена в сервисе", to_node_id);
            return Err(e);
        }
        else
        {
            let res = astar(&graph.graph, i1.unwrap().to_owned(), |g| &g == i2.unwrap(), |e| *e.weight(), |_| 0).unwrap();
            let stations: Vec<Station> = res.1.iter().map(|m| graph.graph.node_weight(*m).unwrap().clone()).collect();
            let time = (res.0 as f32 / 60.0).ceil() as u32;
            return Ok(MetroPath
            {
                time,
                stations
            });
        }
    }
    else 
    {
        let e = "Ошибка загрузки данных из файла map.json".to_owned();
        return Err(e);
    }
}

pub fn find_nearest(target_node_id: &str, nearest_stations_time: u32) -> Result<Vec<Nearest>, String>
{
    if let Some(graph) = GRAPH.as_ref()
    {
        let i1 = graph.get_by_index(target_node_id);
        if i1.is_none()
        {
            let e = format!("Нода с id {} не найдена в сервисе", target_node_id);
            return Err(e);
        }
        else
        {
            let dij = dijkstra(&graph.graph, i1.unwrap().to_owned(),  None, |e| *e.weight());
            let mut nearest: Vec<Nearest> = vec![];
            for v in dij
            {
                if ((v.1 as u32) / 60) < nearest_stations_time && &v.0 != i1.unwrap()
                {
                    nearest.push(Nearest
                    {
                        time: (v.1 as f32 / 60.0).ceil() as u32,
                        station: graph.graph.node_weight(v.0).unwrap().clone()
                    });
                }
            }
            return Ok(nearest);
        }
       
    }
    else
    {
        let e = "Ошибка загрузки данных из файла map.json".to_owned();
        return Err(e);
    }
   
}

pub fn get_stations() -> Result<Vec<Station>, String>
{
    if let Some(graph) = GRAPH.as_ref()
    {
        let stations: Vec<Station> = graph.graph.node_weights().map(|m| m.clone()).collect();
        return Ok(stations);
    }
    else 
    {
        let e = "Ошибка загрузки данных из файла map.json".to_owned();
        return Err(e);
    }
}

#[cfg(test)]
mod tests
{
    use logger::info;
    
    #[test]
    fn test_find_path()
    {
        logger::StructLogger::initialize_logger();
        let path = super::find_path("nd89811596", "nd77715428");
        info!("{}", path.unwrap());
    }
    #[test]
    fn test_find_nearest()
    {
        logger::StructLogger::initialize_logger();
        let nrst = super::find_nearest("nd89811596", 10).unwrap();
        for n in nrst
        {
            info!("{}", n);
        }
    }
}