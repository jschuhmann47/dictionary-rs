use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WordDefinition {
    pub word: String,
    pub pos: Vec<String>,
    pub verbs: Vec<Verb>,
    pub pronunciation: Vec<Pronunciation>,
    pub definition: Vec<Definition>,
}

impl WordDefinition {
    pub fn get_definitions(&self) -> Vec<String> {
        self.definition.iter().map(|d| d.text.clone()).collect()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pronunciation {
    pub pos: String,
    pub lang: String,
    pub url: String,
    pub pron: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Definition {
    pub id: i64,
    pub pos: String,
    pub source: String,
    pub text: String,
    pub translation: String,
    pub example: Vec<Example>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Example {
    pub id: i64,
    pub text: String,
    pub translation: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Verb {
    pub id: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub text: String,
}
