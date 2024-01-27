use std::collections::HashMap;
use std::ops::Deref;
use crate::parse_json::{Data, Transform};

pub mod parse_json;

fn log_parsed(data: &HashMap<String, Box<Data>>) {
    for (k, v) in data {
        match v.deref() {
            Data::Bool(b) => println!("___ BOOL {:?} at {:?}", b, k),
            Data::Number(n) => println!("___ NUMBER {:?} at {:?}", n, k),
            Data::String(s) => println!("___ STRING {} at {}", s, k),
            Data::Map(map) => log_parsed(&map),
            Data::Array(array) => println!("___ ARRAY {:?} at {}", array, k),
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
