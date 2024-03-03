use crate::xforms::{resolve_source, Transform, TransformSource, Transformable};
use serde::de::{Deserializer, Error};
use serde::Deserialize;
use serde_json::{Map, Value};

#[derive(Debug, PartialEq)]
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

    pub fn resolve_source(&mut self, variables: &Map<String, Value>) -> bool {
        resolve_source(self, variables)
    }
}

impl Transformable for MapTransform {
    fn get_source(&mut self) -> &mut Box<TransformSource> {
        &mut self.source
    }

    fn get_source_value(&self) -> Option<&Value> {
        self.source_value.as_ref()
    }

    fn set_source(&mut self, value: Value) -> () {
        self.source_value = Some(value);
    }

    fn transform(&mut self, variables: &Map<String, Value>) {
        if self.source_value == None {
            self.resolve_source(&variables);
        }

        match &self.source_value {
            Some(Value::Array(arr)) => {
                // TODO: actually map the values with self.mapper
                let mut result: Vec<Value> = Vec::new();
                for item in arr.iter() {
                    result.push(item.clone());
                }
                self.value = Some(serde_json::to_value(result).unwrap());
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
