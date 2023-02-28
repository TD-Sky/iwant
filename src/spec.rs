use serde::Deserialize;
use std::collections::HashMap;

pub type Manifest = HashMap<String, Category>;
pub type Category = HashMap<String, Item>;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Item {
    Description(String),
    Info(Info),
}

#[derive(Debug, Deserialize)]
pub struct Info {
    pub packages: Option<Vec<String>>,
    pub manager: Option<String>,
    pub description: Option<String>,
}
