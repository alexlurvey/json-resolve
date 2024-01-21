use serde::Deserialize;
use serde::de::{Deserializer, Error};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct MapTransform(String, String, Vec<String>);

impl<'de> Deserialize<'de> for MapTransform {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let (xf, src, props): (&str, &str, Vec<String>) = Deserialize::deserialize(deserializer)?;

        if xf == "xf_map" {
            Ok(MapTransform::new(src, props))
        } else {
            Err(D::Error::custom(format!("tried parsing xf_map, found [{:?}, {:?}, {:?}]", xf, src, props)))
        }
    }
}

#[derive(Debug, PartialEq)]
struct PluckTransform(String, String, Vec<String>);

impl<'de> Deserialize<'de> for PluckTransform {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let (xf, src, props): (&str, &str, Vec<String>) = Deserialize::deserialize(deserializer)?;
        
        if xf == "xf_pluck" {
            Ok(PluckTransform::new(src, props))
        } else {
            Err(D::Error::custom(format!("tried parsing xf_map, found [{:?}, {:?}, {:?}]", xf, src, props)))
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Transform {
    MapTransform(MapTransform),
    PluckTransform(PluckTransform),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Data {
    String(String),
    Number(i64),
    Bool(bool),
    Map(HashMap<String, Box<Data>>),
    Tranform(Transform),
    Data(Box<Data>),
}

impl MapTransform {
    pub fn new(source: &str, list: Vec<String>) -> Self {
        MapTransform(String::from("xf_map"), String::from(source), list)
    }
}

impl PluckTransform {
    pub fn new(source: &str, list: Vec<String>) -> Self {
        PluckTransform(String::from("xf_pluck"), String::from(source), list)
    }
}

const JSON: &'static str = r#"
    {
        "obj": {
            "nest": "string",
            "map": ["xf_map", "$data", ["lookup", "my", "data"]],
            "pluck": ["xf_pluck", "$data", ["pluck", "me", "daddy"]],
            "bool": true,
            "num": 42,
            "more_nest": {
                "mapper": ["xf_map", "$data", ["prop"]]
            }
        },
        "num": -98,
        "string": "testing",
        "bool": false,
        "map": ["xf_map", "$source", ["map_property"]],
        "pluck": ["xf_pluck", "$object", ["pluck_property"]]
    }"#;

pub fn resolve(_object: HashMap<&str, &str>) {
    let parsed: HashMap<String, Data> = serde_json::from_str(JSON).expect("error parsing json");

    for (k, v) in &parsed {

        println!("{}: {:?}", k, v);
    }
}
