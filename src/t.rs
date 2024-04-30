use serde::{Deserialize, Serialize};

use serde_json::Value;

pub type Root = Vec<Root2>;
 
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root2 {
    pub word: String,
    pub phonetics: Vec<Phonetic>,
    pub origin: Option<String>,
    pub meanings: Vec<Meaning>,
    pub license: License2,
    pub source_urls: Vec<String>,
}
 
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Phonetic {
    pub audio: String,
    pub source_url: Option<String>,
    pub license: Option<License>,
    pub text: Option<String>,
}
 
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct License {
    pub name: String,
    pub url: String,
}
 
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meaning {
    pub part_of_speech: String,
    pub definitions: Vec<Definition>,
    pub synonyms: Vec<String>,
    pub antonyms: Vec<String>,
}
 
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Definition {
    pub definition: String,
    pub synonyms: Vec<Value>,
    pub antonyms: Vec<Value>,
    pub example: Option<String>,
}
 
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct License2 {
    pub name: String,
    pub url: String,
}
