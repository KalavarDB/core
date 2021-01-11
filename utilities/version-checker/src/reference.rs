use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CrateDirectory {
    pub name: String,
    pub path: String,
    pub sha: String,
    pub size: usize,
    pub url: String,
    #[serde(rename = "type")]
    pub content: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CrateAdvisory {}