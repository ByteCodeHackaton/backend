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

// #[derive(Clone, Serialize, Deserialize, Debug)]
// pub struct MetroGraph2
// {
//     pub graph: Graph<SubwayStation, usize>,
//     indexes: HashMap<String, NodeIndex<u32>>
// }
// impl MetroGraph2
// {
//     pub fn get_by_index(&self, node_id: &str) -> Option<&NodeIndex<u32>>
//     {
//         self.indexes.get(node_id)
//     }
// }


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
    use std::{collections::HashMap, io::Write};

    use logger::info;
    use petgraph::{algo::{self, astar}, data::Build, graph::NodeIndex, visit::{EdgeFiltered, EdgeRef}};

    use crate::{draw::draw_graph, MetroGraph};

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

    //
    // #[test]
    // fn test_generate_new_schema2()
    // {
    //     logger::StructLogger::initialize_logger();
    //     let names = super::extract_data("data/by_task/metro_stations_name.json").unwrap();
    //     let mut indexes: HashMap<String, NodeIndex<u32>> = HashMap::new();
    //     let mut g = petgraph::Graph::<SubwayStation, usize>::new();
    //     for n in names.as_array().unwrap()
    //     {
    //         indexes.insert(n["id"].as_str().unwrap().to_owned(),g.add_node(SubwayStation
    //         {
    //             id: n["id"].as_str().unwrap().to_owned(),
    //             line: n["name_line"].as_str().unwrap().to_owned(),
    //             name: n["name_station"].as_str().unwrap().to_owned()
    //         }));
    //     }
    //     let ride_time = super::extract_data("data/by_task/metro_station_time.json").unwrap();
    //     for r in ride_time.as_array().unwrap()
    //     {
    //         let node_1 = indexes.get(r["id_st1"].as_str().unwrap()).unwrap();
    //         let node_2 = indexes.get(r["id_st2"].as_str().unwrap()).unwrap();
    //         let time: f64 = r["time"].as_str().unwrap().replace(",", ".").parse().unwrap();
    //         let time = (time * 60.0) as usize;
    //         g.update_edge(node_1.to_owned(), node_2.to_owned(), time as usize);
    //         //g.update_edge(node_2.to_owned(), node_1.to_owned(), time as usize);
    //     }
    //     let walk_time = super::extract_data("data/by_task/metro_walk_time.json").unwrap();
    //     for r in walk_time.as_array().unwrap()
    //     {
    //         let node_1 = indexes.get(r["id1"].as_str().unwrap());
    //         let node_2 = indexes.get(r["id2"].as_str().unwrap());
    //         if node_1.is_some() && node_2.is_some()
    //         {
    //             let time: f64 = r["time"].as_str().unwrap().replace(",", ".").parse().unwrap();
    //             let time = (time * 60.0) as usize;
    //             g.update_edge(node_1.unwrap().to_owned(), node_2.unwrap().to_owned(), time as usize);
    //             //g.update_edge(node_2.unwrap().to_owned(), node_1.unwrap().to_owned(), time as usize);
    //         }
    //         else {
    //             logger::error!("{:?} {:?}", node_1, node_2);
    //         }
          
    //     }

        
    //     let serialized_obj = MetroGraph2
    //     {
    //         graph: g.clone(),
    //         indexes: indexes.clone()
    //     };
    //     let json = serde_json::to_string(&serialized_obj).unwrap();
    //     let mut file = std::fs::File::create("map2.json").unwrap();
    //     let _ = file.write_all(&json.as_bytes());


    //     let i1 = serialized_obj.get_by_index("367").unwrap();
    //     let i2 = serialized_obj.get_by_index("352").unwrap();
       
    //     let res = astar(&serialized_obj.graph, i1.to_owned(), |g| &g == i2, |e| *e.weight(), |_| 0).unwrap();
    //     let stations: Vec<SubwayStation> = res.1.iter().map(|m| serialized_obj.graph.node_weight(*m).unwrap().clone()).collect();
    //     let time = (res.0 as f32 / 60.0).ceil() as u32;
    //     logger::info!("{:?} {}", stations, time);
    // }


}