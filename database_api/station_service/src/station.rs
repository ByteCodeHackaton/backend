use std::fmt::Display;
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Eq, Debug, PartialEq, Ord, PartialOrd, Hash)]
pub struct Station
{
    pub node_id: String,
    pub station_id: String,
    pub station_name: String
}

impl Display for Station
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        let node = ["node_id: ", &self.node_id, " "].concat();
        let station = ["station_id: ", &self.station_id, " "].concat();
        let name = ["name: ", &self.station_name].concat();
        let _ = f.write_str(&node);
        let _ = f.write_str(&station);
        f.write_str(&name)
    }
}