use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StatsCfg {
    pub id: String,
    pub title: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cfg {
    pub header: Vec<String>,
    pub skips: Vec<String>,
    pub stats: Vec<StatsCfg>,
}
