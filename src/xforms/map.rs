use crate::xforms::pluck::pluck;
use crate::xforms::{Transform, TransformSource, Transformable};
use serde::de::{Deserializer, Error};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, PartialEq, Serialize)]
pub struct MapTransform {
    pub source: Box<TransformSource>,
    pub mapper: Box<Transform>,
    pub value: Option<Value>,
    pub source_value: Option<Value>,
}

impl MapTransform {
    pub fn new(source: TransformSource, mapper: Box<Transform>) -> Self {
        MapTransform {
            source: Box::new(source),
            mapper,
            value: None,
            source_value: None,
        }
    }
}

impl Transformable for MapTransform {
    fn get_source(&mut self) -> &mut Box<TransformSource> {
        &mut self.source
    }

    fn get_source_value(&self) -> Option<&Value> {
        self.source_value.as_ref()
    }

    fn set_source(&mut self, value: Value) {
        self.source_value = Some(value);
    }

    fn transform(&mut self, variables: &Map<String, Value>) {
        if self.source_value.is_none() {
            self.resolve_source(variables);
        }

        match &self.source_value {
            Some(Value::Array(arr)) => {
                if let Ok(v) = map(arr, &self.mapper) {
                    self.value = Some(v);
                }
            }
            None => {
                println!("attempted to transform {:?} but found None", self.source);
            }
            other => {
                println!("xf_map source resolved to a non-array value {:?}", other);
            }
        }
    }
}

pub fn map(source: &[Value], mapper: &Transform) -> Result<serde_json::Value, serde_json::Error> {
    let mut result: Vec<Value> = Vec::new();

    for item in source.iter() {
        match mapper {
            Transform::Pluck(ref xf) => {
                if let Value::Object(obj) = item {
                    if let Some(v) = pluck(obj, &xf.path) {
                        result.push(v.clone());
                    }
                }
            }
            Transform::Map(ref xf) => {
                if let Value::Array(array) = item {
                    if let Ok(mapped) = map(array, &xf.mapper) {
                        if let Ok(v) = serde_json::to_value(mapped) {
                            result.push(v);
                        }
                    }
                }
            }
        }
    }

    serde_json::to_value(result)
}

impl<'de> Deserialize<'de> for MapTransform {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let (xf, src, mapper): (&str, TransformSource, Transform) =
            Deserialize::deserialize(deserializer)?;

        if xf == "xf_map" {
            Ok(MapTransform::new(src, Box::new(mapper)))
        } else {
            Err(D::Error::custom(format!(
                "tried parsing xf_map, found [{:?}, {:?}, {:?}]",
                xf, src, mapper
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::__resolve;
    use crate::xforms::Transform;
    use crate::Data;
    use serde_json::json;
    use serde_json::Number;
    use serde_json::{Map, Value};

    #[test]
    fn xf_map_with_xf_pluck() {
        let json = r#"
            { "a_map": ["xf_map", "$src", ["xf_pluck", "$", ["prop"]]], "some_string": "string_value" }
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

        let result = __resolve(json, &variables);

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

    #[test]
    fn xf_map_with_xf_map() {
        let json = r#"
            { "a_map": ["xf_map", "$src", ["xf_map", "$", ["xf_pluck", "$", ["count"]]]] }
        "#;

        let variables: Map<String, Value> = json!({
            "src": [
                [{ "count": 1 }, { "count": 2 }, { "count": 3 }],
                [{ "count": 4 }, { "count": 5 }, { "count": 6 }]
            ]
        })
        .as_object()
        .unwrap()
        .to_owned();

        let result = __resolve(json, &variables);

        let result = result.get("a_map").unwrap();

        if let Data::Transform(Transform::Map(xf)) = result {
            let result: Vec<Value> = xf
                .value
                .as_ref()
                .expect("value of MapTransform at 'a_map' was not resolved")
                .as_array()
                .expect("value of MapTransform at 'a_map' was not resolved to an array")
                .to_vec();

            let expected: Vec<Value> = vec![
                Value::Array(vec![
                    Value::Number(Number::from(1)),
                    Value::Number(Number::from(2)),
                    Value::Number(Number::from(3)),
                ]),
                Value::Array(vec![
                    Value::Number(Number::from(4)),
                    Value::Number(Number::from(5)),
                    Value::Number(Number::from(6)),
                ]),
            ];

            assert_eq!(result, expected);
        } else {
            panic!("'a_map' was not serialized into a MapTransform");
        }
    }
}
