use super::Parameter;
use serde::Deserialize;

#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct MatchInfo {
    pub num_match_types: u64,
    pub match_type: Vec<MatchType>,
}
#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct MatchType {
    #[serde(rename = "@index")]
    pub index: usize,
    #[serde(rename = "TypeID")]
    pub type_id: String,
    #[serde(default)]
    pub current_index: i32,
    pub num_match_collections: u64,
    #[serde(default)]
    pub match_collection: Vec<MatchCollection>,
}
#[derive(Default, Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct MatchCollection {
    #[serde(rename = "@index")]
    pub index: usize,
    pub core_name: String,
    #[serde(default)]
    pub match_index: i32,
    #[serde(rename = "Parameter")]
    #[serde(default)]
    pub parameters: Vec<Parameter>,
}
