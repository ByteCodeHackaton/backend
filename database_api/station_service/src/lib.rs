use std::sync::{Arc, Mutex};
use std::{collections::HashMap, fs::File, io::BufReader, path::Path};
use logger::{error, info};
use petgraph::graph::UnGraph;
use rustworkx_core::petgraph::algo::{dijkstra, floyd_warshall};
use rustworkx_core::petgraph::data::{Build, DataMap};
use rustworkx_core::petgraph::graph::{DiGraph, Node, NodeIndex};
use rustworkx_core::petgraph::visit::{EdgeIndexable, EdgeRef, IntoNodeIdentifiers, IntoNodeReferences, NodeRef};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use once_cell::sync::Lazy;
use rustworkx_core::petgraph::Graph;
use rustworkx_core::centrality::betweenness_centrality;


static LABELS : Lazy<Mutex<HashMap<String, String>>> = Lazy::new(|| Mutex::new(HashMap::new()));
static NODEID_STATIONID : Lazy<Mutex<HashMap<String, String>>> = Lazy::new(|| Mutex::new(HashMap::new()));
static NODEID_NODEINDEX : Lazy<Mutex<HashMap<NodeIndex<u32>, String>>> = Lazy::new(|| Mutex::new(HashMap::new()));
static NODEID_NODEINDEX_REVERSE : Lazy<Mutex<HashMap<NodeIndex<u32>, String>>> = Lazy::new(|| Mutex::new(HashMap::new()));

#[derive(Serialize, Deserialize, Clone)]
pub struct StationObject<'a> where Self: DeserializeOwned
{
    stations_dict: HashMap<&'a str, &'a str>,
    labels: HashMap<&'a str, &'a str>,
    node_indexes: HashMap<&'a str, usize>,
    ///a, b, weight
    edges: Vec<(usize, usize, usize)>
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

fn get_graph()
{
    let mut stations_dict : HashMap<&str, &str> = HashMap::new();
    let mut labels : HashMap<&str, &str> = HashMap::new();
    let mut node_indexes : HashMap<&str, NodeIndex<u32>> = HashMap::new();
    let mut node_indexes_reverse : HashMap<NodeIndex<u32>, &str> = HashMap::new();
    let mut so = StationObject
    {
        labels: labels.clone(),
        stations_dict: stations_dict.clone(),
        node_indexes: HashMap::new(),
        edges: vec![]

    };
    let names = extract_data("data/names.json").unwrap();
    let graph = extract_data("data/data.json").unwrap();
    for stop in graph["stops"]["items"].as_array().unwrap()
    {
        stations_dict.insert(stop["nodeId"].as_str().unwrap(), stop["stationId"].as_str().unwrap());
    }
    let mut g = Graph::<&str, usize>::new();
    for node in graph["nodes"]["items"].as_array().unwrap()
    {
        let id = node["id"].as_str().unwrap();
        if stations_dict.contains_key(id)
        {
            let node_index = g.add_node(id);
            node_indexes.insert(id,  node_index);
            so.node_indexes.insert(id,  node_index.index());
            node_indexes_reverse.insert(node_index, id);
        }
    }
    for link in graph["links"]["items"].as_array().unwrap()
    {
        let a = node_indexes.get(link["fromNodeId"].as_str().unwrap());
        let b = node_indexes.get(link["toNodeId"].as_str().unwrap());
        if a.is_some() && b.is_some()
        {
            let len = link["attributes"]["time"].as_u64().unwrap() as usize;
            so.edges.push((a.unwrap().index(), b.unwrap().index(), len));
            g.add_edge(a.unwrap().to_owned(), b.unwrap().to_owned(), len);
        }
    }
    for id in g.raw_nodes()
    {
        if let Some(int_name) = stations_dict.get(&id.weight)
        {
            let node_name = [int_name, "-name"].concat();
            if let Some(label) = names.get("keysets").and_then(|g|g.get("generated").and_then(|d| d.get(node_name).and_then(|ru| ru.get("ru"))))
            {
                labels.insert(id.weight, label.as_str().unwrap());
            }
        } 
    }
    so.labels = labels.clone();
    
    let i1 = node_indexes.get("nd89811596").unwrap();
    let i2 = node_indexes.get("nd77715428").unwrap();
    let dij = dijkstra(&g, i1.to_owned(),  Some(i2.to_owned()), |e| *e.weight());
    for v in dij
    {
        //let node_str_index = node_indexes_reverse.get(&v.0).unwrap();
        //info!("{}, {}", v.1, labels.get(node_str_index).unwrap());
        if &v.0 == i2
        {
            info!("{} -> {} = {}",labels.get("nd89811596").unwrap(), labels.get("nd77715428").unwrap(), (v.1 as f32 / 60.0).ceil());
        }
    }

}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Station
{
    node_id: String,
    station_id: String,
    station_name: String
}

struct MetroMap
{
    map: petgraph::Graph<Station, usize>
}

fn generate_new_shema()
{
    let mut stations_dict : HashMap<&str, &str> = HashMap::new();
    let mut labels : HashMap<&str, &str> = HashMap::new();
    let mut node_indexes : HashMap<&str, NodeIndex<u32>> = HashMap::new();
    let mut node_indexes_reverse : HashMap<NodeIndex<u32>, &str> = HashMap::new();
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
        if let Some(node) = graph["nodes"]["items"].as_array().unwrap().iter().find(|f| f["id"] == station_id)
        {
            let station = Station
            {
                station_id: station_id.to_owned(),
                node_id: node_id.to_owned(),
                station_name: name
            };
            let index = g.add_node(station);
            node_indexes.insert(node_id, index);
        }
    }

    
    // let mut g = Graph::<&str, usize>::new();
    // for node in graph["nodes"]["items"].as_array().unwrap()
    // {
    //     let id = node["id"].as_str().unwrap();
    //     if stations_dict.contains_key(id)
    //     {
    //         let node_index = g.add_node(id);
    //         node_indexes.insert(id,  node_index);
    //         node_indexes_reverse.insert(node_index, id);
    //     }
    // }
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
    // for id in g.raw_nodes()
    // {
    //     if let Some(int_name) = stations_dict.get(&id.weight)
    //     {
    //         let node_name = [int_name, "-name"].concat();
    //         if let Some(label) = names.get("keysets").and_then(|g|g.get("generated").and_then(|d| d.get(node_name).and_then(|ru| ru.get("ru"))))
    //         {
    //             labels.insert(id.weight, label.as_str().unwrap());
    //         }
    //     } 
    // }
    // so.labels = labels.clone();
    
    let i1 = node_indexes.get("nd89811596").unwrap();
    let i2 = node_indexes.get("nd77715428").unwrap();
    let dij = dijkstra(&g, i1.to_owned(),  Some(i2.to_owned()), |e| *e.weight());
    for v in dij
    {
        //let node_str_index = node_indexes_reverse.get(&v.0).unwrap();
        //info!("{}, {}", v.1, labels.get(node_str_index).unwrap());
        if &v.0 == i2
        {
            info!("{} -> {} = {}",labels.get("nd89811596").unwrap(), labels.get("nd77715428").unwrap(), (v.1 as f32 / 60.0).ceil());
        }
    }
    plotters:: draw_graph(&g);

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
    fn test_get_graph()
    {
        logger::StructLogger::initialize_logger();
        super::get_graph();
    }
}


// names = json.loads(codecs.open( "l10n.json", "r", "utf_8_sig" ).read())
// graph = json.loads(codecs.open( "data.json", "r", "utf_8_sig" ).read())nodeStdict={}

// for stop in graph['stops']['items']:
//     nodeStdict[stop['nodeId']]=stop['stationId']

// G=nx.Graph()
// for node in graph['nodes']['items']:
//     G.add_node(node['id'])
// for link in graph['links']['items']:
//     G.add_edge(link['fromNodeId'], link['toNodeId'], length=link['attributes']['time'])
// nodestoremove=[]
// for node in G.nodes():
//     if len(G.edges(node))<2:
//         nodestoremove.append(node)
// for node in nodestoremove:
//     G.remove_node(node)
// labels={}
// for node in G.nodes():
//     try:
//         labels[node]=names['keysets']['generated'][nodeStdict[node]+'-name']['ru']
//     except: 
// 				labels[node]='error'