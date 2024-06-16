use std::fmt::Display;
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MetroPath
{
    pub time: u32,
    pub stations: Vec<super::Station>
}
impl Display for MetroPath
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        let joins = self.stations.iter().map(|m| m.station_name.clone()).collect::<Vec<String>>().join("->");
        let formatted = format!("`{}` примерно {} минут", &joins, self.time.to_string());
        f.write_str(&formatted)
    }
}