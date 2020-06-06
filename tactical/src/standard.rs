use crate::traits::*;
use crate::tree;
use serde::{Deserialize, Deserializer, de};
use serde::de::{DeserializeOwned, Error};
use serde_json::Value;

pub fn node_properties<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: DeserializeOwned,
        D: Deserializer<'de>
{
    #[derive(Debug, Deserialize)]
    struct Pair {
        name: String,
        value: Value,
    }

    fn default_options() -> Vec<Pair> {
        vec![]
    }

    #[derive(Debug, Deserialize)]
    struct Properties {
        #[serde(default="default_options")]
        properties: Vec<Pair>
    };

    // Deserialize a Vec<String> and Vec<Value>.
    let nv = Properties::deserialize(deserializer)?;

    // Zip them together into a map.
    let pairs = Value::Object(nv.properties.iter().map(|f| (f.name.clone(), f.value.clone())).collect());

    // Deserialize the output type T.
    T::deserialize(pairs).map_err(|err| de::Error::custom(err.to_string()))
}


/// # Standard nodes
///

/// ## Sequence
///
#[derive(Debug, Deserialize)]
pub struct Sequence {}
impl Node for Sequence {

}

/// ## Select
#[derive(Debug, Deserialize)]
pub struct Selector {}
impl Node for Selector {

}

/// ## Subtree
/// Loads a specified path as a behaviour tree.
/// Ticking this node acts as if the root node of the subtree was ticked
///
#[derive(Debug, Deserialize)]
pub struct Subtree {
    #[serde(alias="Path", deserialize_with="subtree_path")]
    path: Box<tree::Node>
}
impl Node for Subtree {
    fn tick(&self) -> Status {
        self.path.tick()
    }
}
fn subtree_path<'de, D>(deserializer: D) -> Result<Box<tree::Node>, D::Error>
    where
        D: Deserializer<'de>
{
    let path: String = Deserialize::deserialize(deserializer)?;
    Ok(Box::new(tree::Node::from_file(path.as_str()).map_err(
        |err| de::Error::custom(err.to_string()))?
    ))
}


/// # Standard services
///

/// ## Comment
#[derive(Debug, Deserialize)]
pub struct Comment {
    #[serde(alias="Comment")]
    comment: String
}
impl Service for Comment {

}

/// # Standard decorators

/// ## Failure
#[derive(Debug, Deserialize)]
pub struct Failure {}
impl Decorator for Failure {

}

/// ## Success
#[derive(Debug, Deserialize)]
pub struct Success {}
impl Decorator for Success {

}

/// ## Invert
#[derive(Debug, Deserialize)]
pub struct Invert {}
impl Decorator for Invert {

}

