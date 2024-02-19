use super::Parameter;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct MatchInfo {
    pub num_match_types: u64,
    pub match_type: Vec<MatchType>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct MatchType {
    #[serde(rename = "@index")]
    pub index: usize,
    #[serde(rename = "TypeID")]
    pub type_id: String,
    pub current_index: Option<i32>,
    pub num_match_collections: u64,
    pub match_collection: Option<Vec<MatchCollection>>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct MatchCollection {
    #[serde(rename = "@index")]
    pub index: usize,
    pub core_name: String,
    pub match_index: Option<i32>,
    #[serde(rename = "Parameter")]
    pub parameters: Option<Vec<Parameter>>,
}
