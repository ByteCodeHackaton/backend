use std::fmt::Display;
use std::io::Write;
use std::{collections::HashMap, fs::File, io::BufReader, path::Path};
use logger::{error, info};
use once_cell::sync::Lazy;
use petgraph::algo::{dijkstra, astar};
use petgraph::graph::NodeIndex;
use petgraph::Graph;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

use crate::draw::draw_graph;

use super::Station;


pub static GRAPH : Lazy<Option<MetroGraph>> = Lazy::new(|| load_map());

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MetroGraph
{
    pub graph: Graph<Station, usize>,
    indexes: HashMap<String, NodeIndex<u32>>
}
impl MetroGraph
{
    pub fn get_by_index(&self, node_id: &str) -> Option<&NodeIndex<u32>>
    {
        self.indexes.get(node_id)
    }
}


fn extract_data<P: AsRef<Path>>(path: P) -> Option<Value>
{
    let f = File::open(path);
    if f.is_err()
    {
        error!("{}", f.err().unwrap());
        return None;
    }
    let reader = BufReader::new(f.unwrap());
    let reader = serde_json::from_reader(reader);
    Some(reader.unwrap())
}
fn load_map() -> Option<MetroGraph>
{
    
    // let mut f = File::open("map.json");
    // if f.is_err()
    // {
    //     info!("Схема map.json не найдена, попытка сформировать новую схему графа");
    //     generate_new_schema(false);
    //     f = File::open("map.json");
    //     if f.is_err()
    //     {
    //         error!("{}", f.err().unwrap());
    //         return None;
    //     }
    // }
    let f = include_bytes!("../map.json");
    let reader = BufReader::new(f.as_slice());
    let reader = serde_json::from_reader(reader);
    Some(reader.unwrap())
}

fn generate_new_schema(draw_dia: bool)
{
    let mut stations_dict : HashMap<&str,Station> = HashMap::new();
    let mut node_indexes : HashMap<String, NodeIndex<u32>> = HashMap::new();
    let names = extract_data("data/names.json").unwrap();
    let graph = extract_data("data/data.json").unwrap();
    let mut g = petgraph::Graph::<Station, usize>::new();
    for stop in graph["stops"]["items"].as_array().unwrap()
    {
        let node_id = stop["nodeId"].as_str().unwrap();
        let station_id = stop["stationId"].as_str().unwrap();
        let mut name = String::new();
        let node_name = [station_id, "-name"].concat();
        if let Some(label) = names.get("keysets").and_then(|g|g.get("generated").and_then(|d| d.get(node_name).and_then(|ru| ru.get("ru"))))
        {
            name = label.as_str().unwrap().into();
        }
        let station = Station
        {
            station_id: station_id.to_owned(),
            node_id: node_id.to_owned(),
            station_name: name
        };
        stations_dict.insert(node_id, station);
    }
    for node in graph["nodes"]["items"].as_array().unwrap()
    {
        let id = node["id"].as_str().unwrap();
        if let Some(id) = stations_dict.get(id)
        {
            let node_index = g.add_node(id.clone());
            node_indexes.insert(id.node_id.clone(),  node_index);
        }
    }
    for link in graph["links"]["items"].as_array().unwrap()
    {
        let a = node_indexes.get(link["fromNodeId"].as_str().unwrap());
        let b = node_indexes.get(link["toNodeId"].as_str().unwrap());
        if a.is_some() && b.is_some()
        {
            let len = link["attributes"]["time"].as_u64().unwrap() as usize;
            g.add_edge(a.unwrap().to_owned(), b.unwrap().to_owned(), len);
        }
    }
    if draw_dia
    {
        draw_graph(&g)
    }
    let serialized_obj = MetroGraph
    {
        graph: g,
        indexes: node_indexes
    };
    let json = serde_json::to_string(&serialized_obj).unwrap();
    let mut file = std::fs::File::create("map.json").unwrap();
    let _ = file.write_all(&json.as_bytes());
    // let i1 = node_indexes.get("nd89811596").unwrap();
    // let i2 = node_indexes.get("nd77715428").unwrap();
    // let res = petgraph::algo::astar(&g, i1.to_owned(), |g| &g == i2, |e| *e.weight(), |_| 0).unwrap();
    // for ni in res.1
    // {
    //     info!("astar algo: {:?} = {}", &g.node_weight(ni).as_ref().unwrap().station_name, (res.0 as f32 / 60.0).ceil());
    // }
    
    // let dij = dijkstra(&g, i1.to_owned(),  Some(i2.to_owned()), |e| *e.weight());
    // for v in dij
    // {
    //     //let node_str_index = node_indexes_reverse.get(&v.0).unwrap();
    //     //info!("{}, {}", v.1, labels.get(node_str_index).unwrap());
    //     if &v.0 == i2
    //     {
    //         info!("{} -> {} = {}", &g.node_weight(*i1).as_ref().unwrap().station_name, &g.node_weight(*i2).as_ref().unwrap().station_name, (v.1 as f32 / 60.0).ceil());
    //     }
    // }
   
}

#[cfg(test)]
mod tests
{
    #[test]
    fn test_extract_data()
    {
        logger::StructLogger::initialize_logger();
        super::extract_data("data/data.json");
    }
    #[test]
    fn test_generate_new_schema()
    {
        logger::StructLogger::initialize_logger();
        super::generate_new_schema(false);
    }
}