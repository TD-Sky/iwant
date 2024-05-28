use std::collections::HashMap;

use serde::Deserialize;
use smol_str::SmolStr;

use crate::item::CargoKind;

#[derive(Debug, Deserialize)]
pub struct Manifest {
    #[serde(default, rename = "-")]
    pub options: Options,

    #[serde(flatten)]
    pub categories: HashMap<SmolStr, Category>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct Options {
    pub cargo: CargoKind,
}

pub type Category = HashMap<SmolStr, Item>;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Item {
    Desc(String),
    Info(Info),
}

#[derive(Debug, Deserialize)]
pub struct Info {
    pub packages: Option<Vec<SmolStr>>,
    pub manager: Option<SmolStr>,
    pub description: Option<String>,
}
