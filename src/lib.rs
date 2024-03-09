use crate::xforms::{Transform, Transformable};
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
        Data::Transform(xf) => match xf {
            Transform::Map(map) => {
                map.resolve_source(variables);
                if map.source_value.is_some() {
                    map.transform(variables);
                }
            }
            Transform::Pluck(pluck) => {
                pluck.resolve_source(variables);
                if pluck.source_value.is_some() {
                    pluck.transform(variables);
                }
            }
        },
        Data::Map(map) => {
            resolve_map(map, variables);
        }
        Data::Array(array) => {
            resolve_array(array, variables);
        }
        Data::Bool(..) => {
            //    println!("process bool");
        }
        Data::Number(..) => {
            //    println!("process number");
        }
        Data::String(..) => {
            //    println!("process string");
        }
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
    let mut parsed: HashMap<String, Data> =
        serde_json::from_str(json).expect("error parsing json");
    resolve_map(&mut parsed, variables);
    //    println!("{:#?}", parsed);
    parsed
}

#[cfg(test)]
mod tests {
    use crate::resolve;
    use crate::xforms::Transform;
    use crate::Data;
    use serde_json::json;
    use serde_json::Number;
    use serde_json::{Map, Value};

    #[test]
    fn xf_map_resolves_via_xf_pluck() {
        let json = r#"
            { "a_map": ["xf_map", "$src", ["xf_pluck", "$", ["prop"]]] }
        "#;
        let variables: Map<String, Value> = json!({
            "src": [
                { "prop": 1 },
                { "prop": 2 }
            ]
        })
        .as_object()
        .unwrap()
        .to_owned();
        let result = resolve(json, &variables);
        let result = result.get("a_map").unwrap();

        if let Data::Transform(Transform::Map(xf)) = result {
            let result: Vec<Value> = xf
                .value
                .as_ref()
                .expect("value of MapTransform at 'a_map' was not resolved")
                .as_array()
                .expect("value of MapTransform at 'a_map' was not resolved to an array")
                .to_vec();
            let expected = vec![
                Value::Number(Number::from(1)),
                Value::Number(Number::from(2)),
            ];
            assert_eq!(result, expected);
        } else {
            panic!("'a_map' was not serialized into a MapTransform");
        }
    }
}
