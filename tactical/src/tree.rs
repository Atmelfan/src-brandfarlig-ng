use std::fs;
use serde::{Deserialize, Deserializer, de};

use crate::traits;
use crate::owl_bt;

type Error = Box<dyn std::error::Error>;

#[derive(Debug, Deserialize)]
pub struct Node {
    #[serde(flatten)]
    pub(crate) typ: owl_bt::Nodes,
    #[serde(rename="childNodes")]
    children: Option<Vec<Node>>,
    services: Option<Vec<owl_bt::Services>>

}

#[derive(Debug, Deserialize)]
pub struct Tree {
    #[serde(flatten)]
    root: Node,
    name: String
}

impl traits::Node for Node {
    fn tick(&self) -> traits::Status {
        self.typ.tick()
    }
}

impl traits::Node for Tree {
    fn tick(&self) -> traits::Status {
        self.root.tick()
    }
}

impl Node {
    pub fn from_string(json: &str) -> Result<Self, Error> {
        let bt = serde_json::from_str(json)?;
        Ok(bt)
    }

    pub fn from_file(path: &str) -> Result<Self, Error> {
        let json = fs::read_to_string(path)?;
        let bt = Self::from_string(json.as_str())?;
        Ok(bt)
    }
}

impl Tree {
    pub fn from_string(json: &str) -> Result<Self, Error> {
        let bt = serde_json::from_str(json)?;
        Ok(bt)
    }

    pub fn from_file(path: &str) -> Result<Self, Error> {
        let json = fs::read_to_string(path)?;
        let bt = Self::from_string(json.as_str())?;
        Ok(bt)
    }
}


