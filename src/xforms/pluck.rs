use crate::xforms::{resolve_source, TransformSource, Transformable};
use serde::de::{Deserializer, Error};
use serde::Deserialize;
use serde_json::{Map, Value};

#[derive(Debug, PartialEq)]
pub struct PluckTransform {
    pub source: Box<TransformSource>,
    pub path: Box<Vec<String>>,
    pub value: Option<Value>,
    pub source_value: Option<Value>,
}

impl PluckTransform {
    pub fn new(source: TransformSource, path: Box<Vec<String>>) -> Self {
        PluckTransform {
            source: Box::new(source),
            path,
            value: None,
            source_value: None,
        }
    }

    pub fn resolve_source(&mut self, variables: &Map<String, Value>) -> bool {
        resolve_source(self, variables)
    }
}

impl Transformable for PluckTransform {
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
        let found = resolve_source(self, variables);
        if found == true {
            self.value = match self.get_source_value() {
                Some(Value::Object(obj)) => {
                    let mut result: Option<&Value> = None;
                    let mut look: Option<&Map<String, Value>> = Some(obj);

                    for k in self.path.iter() {
                        result = look.expect("should be serde_json::Value::Object").get(k);
                        look = match result {
                            Some(Value::Object(obj)) => Some(obj),
                            _ => look,
                        }
                    }

                    result.cloned()
                }
                None => {
                    println!("xf_pluck source resolved to null");
                    None
                }
                other => {
                    println!("xf_pluck source resolved to non object value, {:?}", other);
                    None
                }
            };
        }
    }
}

impl<'de> Deserialize<'de> for PluckTransform {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let (xf, src, path): (&str, TransformSource, Vec<String>) =
            Deserialize::deserialize(deserializer)?;

        if xf == "xf_pluck" {
            Ok(PluckTransform::new(src, Box::new(path)))
        } else {
            Err(D::Error::custom(format!(
                "tried parsing xf_map, found [{:?}, {:?}, {:?}]",
                xf, src, path
            )))
        }
    }
}
