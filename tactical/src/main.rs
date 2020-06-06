use std::fs;
use enum_dispatch::enum_dispatch;
use serde::Deserialize;

mod traits;
use traits::Node;
mod owl_bt;
mod tree;
mod standard;

fn main() {


    let tree = tree::Tree::from_file("src/test.json")
        .expect("Failed to load behaviour tree file");
    println!("{:#?}", tree);
    tree.tick();


}