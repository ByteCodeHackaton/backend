use serde_derive::{Deserialize, Serialize};


#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Nearest
{
    pub time: u32,
    pub station: super::Station
}

impl std::fmt::Display for Nearest
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        let joins = [&self.station.station_name, "->", &self.time.to_string()].concat();
        f.write_str(&joins)
    }
}