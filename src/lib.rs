use crate::xforms::{Transform, TransformSource, Transformable};
use serde::Deserialize;
use serde_json::{Map, Value};
use std::collections::HashMap;

mod xforms;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Data {
    String(String),
    Number(i64),
    Bool(bool),
    Map(HashMap<String, Data>),
    Transform(Transform),
    Array(Vec<Data>),
}

fn resolve_data(data: &mut Data, variables: &Map<String, Value>) {
    match data {
        Data::Transform(xf) => {
            xf.resolve_source(variables);
            xf.transform(variables);
        }
        Data::Map(map) => {
            resolve_map(map, variables);
        }
        Data::Array(array) => {
            resolve_array(array, variables);
        }
        Data::Bool(..) => {}
        Data::Number(..) => {}
        Data::String(..) => {}
    }
}

fn resolve_array(data: &mut Vec<Data>, variables: &Map<String, Value>) {
    for item in data {
        resolve_data(item, variables);
    }
}

fn resolve_map(data: &mut HashMap<String, Data>, variables: &Map<String, Value>) {
    for (_k, v) in data.iter_mut() {
        resolve_data(v, variables);
    }
}

pub fn resolve(json: &'static str, variables: &Map<String, Value>) -> HashMap<String, Data> {
    let mut parsed: HashMap<String, Data> = serde_json::from_str(json).expect("error parsing json");
    resolve_map(&mut parsed, variables);
    parsed
}

impl Transformable for Data {
    fn get_source(&mut self) -> &mut Box<TransformSource> {
        match *self {
            Data::Transform(ref mut xf) => xf.get_source(),
            _ => unimplemented!(),
        }
    }

    fn get_source_value(&self) -> Option<&Value> {
        match *self {
            Data::Transform(ref xf) => xf.get_source_value(),
            _ => unimplemented!(),
        }
    }

    fn set_source(&mut self, value: Value) {
        match *self {
            Data::Transform(ref mut xf) => xf.set_source(value),
            _ => unimplemented!(),
        }
    }

    fn resolve_source(&mut self, variables: &Map<String, Value>) {
        match *self {
            Data::Transform(ref mut xf) => xf.resolve_source(variables),
            _ => unimplemented!(),
        }
    }

    fn transform(&mut self, variables: &Map<String, Value>) {
        match *self {
            Data::Transform(ref mut xf) => xf.transform(variables),
            _ => unimplemented!(),
        }
    }
}
