use crate::parse_json::{Data, Transform};
use serde_json::{Map, Value};
use std::{collections::HashMap, ops::DerefMut};

pub mod parse_json;

fn resolve_data(data: &mut Box<Data>, variables: &Map<String, Value>) {
    match data.deref_mut() {
        Data::Transform(xf) => {
            match xf {
                Transform::Map(map) => {
                    map.resolve_source(variables);
                },
                Transform::Pluck(pluck) => {
                    pluck.resolve_source(variables);
                }
            }
        },
        Data::Map(map) => {
            resolve_map(map, variables);
        },
        Data::Array(array) => {
            resolve_array(array, variables);
        },
        Data::Bool(..) => {
        //    println!("process bool");
        },
        Data::Number(..) => {
        //    println!("process number");
        },
        Data::String(..) => {
        //    println!("process string");
        },
    }
}

fn resolve_array(data: &mut Vec<Box<Data>>, variables: &Map<String, Value>) {
    for item in data {
        resolve_data(item, variables);
    }
}

fn resolve_map(data: &mut HashMap<String, Box<Data>>, variables: &Map<String, Value>) {
    for (_k, v) in data {
        resolve_data(v, variables);
    }
}

pub fn resolve(json: &'static str, variables: &Map<String, Value>) {
    let mut parsed: HashMap<String, Box<Data>> = serde_json::from_str(json).expect("error parsing json");
    resolve_map(&mut parsed, variables);
    println!("{:#?}", parsed);
}
