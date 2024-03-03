use crate::parse_json::{Data, Transform, Transformable};
use serde_json::{Map, Value};
use std::{collections::HashMap, ops::DerefMut};

pub mod parse_json;

fn resolve_data(data: &mut Box<Data>, variables: &Map<String, Value>) {
    match data.deref_mut() {
        Data::Transform(xf) => {
            match xf {
                Transform::Map(map) => {
                    map.resolve_source(variables);
                    if !map.source_value.is_none() {
                        map.transform(variables);
                    }
                },
                Transform::Pluck(pluck) => {
                    pluck.resolve_source(variables);
                    if !pluck.source_value.is_none() {
                        pluck.transform(variables);
                    }
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

pub fn resolve(json: &'static str, variables: &Map<String, Value>) -> HashMap<String, Box<Data>> {
    let mut parsed: HashMap<String, Box<Data>> = serde_json::from_str(json).expect("error parsing json");
    resolve_map(&mut parsed, variables);
//    println!("{:#?}", parsed);
    parsed
}

#[cfg(test)]
mod tests {
    use crate::resolve;
    use serde_json::json;
    use serde_json::{Map, Value};

    #[test]
    fn it_works() {
        let json = r#"
            { "a_map": ["xf_map", "$src", ["xf_pluck", "$", ["prop"]]] }
        "#;
        let variables: Map<String, Value> = json!({
            "src": [
                { "prop": 1 },
                { "prop": 2 }
            ]
        }).as_object().unwrap().to_owned();
        let result = resolve(json, &variables);    
        println!("res {:#?}", result);
        assert_eq!(true, true);
    }
}
