use std::fs;
use serde::Deserialize;
use std::io::Write;
use heck::{SnakeCase, CamelCase};
use std::collections::{HashMap, BTreeSet};

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum Property {
    #[serde(alias = "string")]
    String {
        name: String,
        default: Option<String>
    },
    #[serde(alias = "number")]
    Number {
        name: String,
        default: Option<isize>,
        max: Option<isize>,
        min: Option<isize>
    },
    #[serde(alias = "bool")]
    Bool {
        name: String,
        default: Option<bool>
    },
    #[serde(alias = "enum")]
    Enum {
        name: String,
        default: Option<String>,
        values: Option<Vec<String>>
    }
}

impl Property {

    fn to_member(&self) -> String {
        let (name, typ) = match self {
            Property::String {name, default: _} => (name, "String"),
            Property::Number {name, default: _, max: _, min: _} => (name, "isize"),
            Property::Bool {name, default: _} => (name, "bool"),
            Property::Enum {name, default: _, values: _} => (name, "String")
        };

        format!("\t#[serde(alias=\"{}\")]\n\t{}: {}",
                name, name.to_snake_case(), typ
        )
    }
}

fn default_module() -> String {
    "standard".to_string()
}

#[derive(Debug, Deserialize)]
struct Node {
    name: String,
    #[serde(default = "default_module")]
    module: String,
    #[serde(alias = "isComposite")]
    is_composite: bool
}

impl Node {
    fn to_variant(&self) -> String {
        format!("\t#[serde(alias = \"{}\",deserialize_with = \"node_properties\")]
\t{}", self.name, self.name.to_camel_case())
    }
}

#[derive(Debug, Deserialize)]
struct Decorator {
    name: String,
    #[serde(default = "default_module")]
    module: String,
    properties: Option<Vec<Property>>
}

impl Decorator {
    fn to_variant(&self) -> String {
        format!("\t#[serde(alias = \"{}\",deserialize_with = \"node_properties\")]
\t{}", self.name, self.name.to_camel_case())
    }
}

#[derive(Debug, Deserialize)]
struct Service {
    name: String,
    #[serde(default = "default_module")]
    module: String,
    properties: Option<Vec<Property>>
}

impl Service {
    fn to_variant(&self) -> String {
        format!("\t#[serde(alias = \"{}\",deserialize_with = \"node_properties\")]
\t{}", self.name, self.name.to_camel_case())
    }
}

#[derive(Debug, Deserialize)]
struct OwlBt {
    nodes: Vec<Node>,
    decorators: Vec<Decorator>,
    services: Vec<Service>,
    properties: Option<Vec<Property>>
}

const HEADER: &str = r"// Autogenerated file, do not edit.
use serde::Deserialize;
use enum_dispatch::enum_dispatch;
use crate::standard::node_properties;
use crate::traits::{
	Node as Node,
	Service as Service,
	Decorator as Decorator,
	Status as Status,
};


";

fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=src/owl-bt.json");
    // Use the `cc` crate to build a C file and statically link it.
    let the_file = fs::read_to_string("src/owl-bt.json").expect("Missing behaviour tree file");
    let owl_bt: OwlBt = serde_json::from_str(the_file.as_str()).expect("JSON was not well-formatted");
    println!("{:?}", owl_bt);

    let mut file = match fs::File::create("src/owl_bt.rs") {
        Err(why) => panic!("Couldn't create src/owl_bt.rs: {}", why.to_string()),
        Ok(file) => file,
    };

    // Header
    file.write_all(HEADER.as_bytes()).expect("");

    //Extract which modules to import
    let mut modules: HashMap<&str, BTreeSet<String>> = HashMap::new();
    for node in &owl_bt.nodes {
        let module = node.module.as_str();
        let x = modules.entry(module).or_default();
        x.insert(node.name.to_camel_case());
    }
    for service in &owl_bt.services {
        let module = service.module.as_str();
        let x = modules.entry(module).or_default();
        x.insert(service.name.to_camel_case());
    }
    for decorator in &owl_bt.decorators {
        let module = decorator.module.as_str();
        let x = modules.entry(module).or_default();
        x.insert(decorator.name.to_camel_case());
    }

    for (module, imports) in modules.iter() {
        file.write_all(format!("use crate::{}::{{\n", module).as_bytes()).unwrap();
        for import in imports {
            file.write_all(format!("\t{},\n", import).as_bytes()).unwrap();
        }
        file.write_all(format!("}};// {}\n\n", module).as_bytes()).unwrap();
    }


    let variants = owl_bt.nodes.iter().map(|n| n.to_variant()).collect::<Vec<String>>().join(",\n");
    file.write_all(format!("
#[enum_dispatch(Node)]
#[derive(Debug, Deserialize)]
#[serde(tag = \"type\")]
pub enum Nodes {{
{}
}}
", variants).as_bytes()).expect("");

    file.write_all("// *** Services ***".as_bytes()).expect("");
    let variants = owl_bt.services.iter().map(|n| n.to_variant()).collect::<Vec<String>>().join(",\n");
    file.write_all(format!("
#[enum_dispatch(Service)]
#[derive(Debug, Deserialize)]
#[serde(tag = \"type\")]
pub enum Services {{
{}
}}
", variants).as_bytes()).expect("");

    file.write_all("// *** Decorators ***".as_bytes()).expect("");
    let variants = owl_bt.decorators.iter().map(|n| n.to_variant()).collect::<Vec<String>>().join(",\n");
    file.write_all(format!("
#[enum_dispatch(Decorator)]
#[derive(Debug, Deserialize)]
#[serde(tag = \"type\")]
pub enum Decorators {{
{}
}}
", variants).as_bytes()).expect("");
}