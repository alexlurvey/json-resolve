use std::collections::HashMap;
use std::ops::Deref;
use crate::parse_json::{Data, Transform};

pub mod parse_json;

fn resolve_data(data: &Box<Data>) -> Box<Data> {
    match data.deref() {
        Data::Transform(xf) => {
            match xf {
                Transform::Map(_map) => {
                    Box::new(Data::String("processing xf_map".to_string()))
                },
                Transform::Pluck(_pluck) => {
                    Box::new(Data::String("procesing xf_pluck".to_string()))
                }
            }
        },
        Data::Map(map) => {
            Box::new(Data::Map(resolve_map(&map)))
        },
        Data::Array(array) => {
            Box::new(Data::Array(resolve_array(array)))
        },
        Data::Bool(b) => {
            Box::new(Data::Bool(*b))
        },
        Data::Number(n) => {
            Box::new(Data::Number(*n))
        },
        Data::String(s) => {
            Box::new(Data::String(s.to_string()))
        },
    }
}

fn resolve_array(data: &Vec<Box<Data>>) -> Vec<Box<Data>> {
    let mut result: Vec<Box<Data>> = Vec::new();

    for item in data {
        result.push(resolve_data(&item));
    }

    result
}

fn resolve_map(data: &HashMap<String, Box<Data>>) -> HashMap<String, Box<Data>> {
    let mut result: HashMap<String, Box<Data>> = HashMap::new();

    for (k, v) in data {
        result.insert(k.to_string(), resolve_data(&v));
    }

    result
}

pub fn resolve(json: &'static str) {
    let parsed: HashMap<String, Box<Data>> = serde_json::from_str(json).expect("error parsing json");
    let root = resolve_map(&parsed);
    println!("{:#?}", root);
}
