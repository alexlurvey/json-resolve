use serde::Deserialize;
use serde::de::{Deserializer, Error};
use std::collections::HashMap;
use std::ops::Deref;

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

fn log_parsed(data: &HashMap<String, Box<Data>>) {
    for (k, v) in data {
        match v.deref() {
            Data::Bool(b) => println!("___ BOOL {:?} at {:?}", b, k),
            Data::Number(n) => println!("___ NUMBER {:?} at {:?}", n, k),
            Data::String(s) => println!("___ STRING {} at {}", s, k),
            Data::Map(map) => log_parsed(&map),
            Data::Tranform(xf) => {
                match xf {
                    Transform::MapTransform(map) => println!("___ XF_MAP at {}: lookup {:?} from {}", k, map.2, map.1),
                    Transform::PluckTransform(pluck) => println!("___ XF_MAP at {}: lookup {:?} from {}", k, pluck.2, pluck.1),
                }
            },
        }
    }
}

pub fn resolve(json: &'static str) {
    let parsed: HashMap<String, Box<Data>> = serde_json::from_str(json).expect("error parsing json");

    log_parsed(&parsed);
}
