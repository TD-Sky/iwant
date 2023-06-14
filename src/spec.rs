use serde::Deserialize;
use smol_str::SmolStr;
use std::collections::HashMap;

pub type Manifest = HashMap<SmolStr, Category>;
pub type Category = HashMap<SmolStr, Item>;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Item {
    Description(String),
    Info(Info),
}

#[derive(Debug, Deserialize)]
pub struct Info {
    pub packages: Option<Vec<SmolStr>>,
    pub manager: Option<SmolStr>,
    pub description: Option<String>,
}
