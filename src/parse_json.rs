use serde::Deserialize;
use serde::de::{Deserializer, Error};
use std::collections::HashMap;
use std::ops::DerefMut;
use serde_json::{Map, Value};

pub trait Transformable {
    fn get_source(&mut self) -> &mut Box<TransformSource>;
    fn get_source_value(&self) -> Option<&Value>;
    fn set_source(&mut self, value: Value) -> ();
    fn transform(&mut self, variables: &Map<String, Value>) -> ();
}

fn resolve_source(xform: &mut impl Transformable, variables: &Map<String, Value>) -> bool {
    match xform.get_source().deref_mut() {
        TransformSource::String(s) => {
            if let Some(v) = variables.get(&s[1..]) {
                xform.set_source(v.clone()); // TODO: no clone?
                return true;
            }
        },

        TransformSource::Transform(source_xform) => {
            source_xform.transform(variables);
        },
    };

    false
}

#[derive(Debug, PartialEq)]
pub struct MapTransform {
    pub source: Box<TransformSource>,
    pub mapper: Box<Transform>,
    pub value: Option<Value>,
    pub source_value: Option<Value>,
}

#[derive(Debug, PartialEq)]
pub struct PluckTransform {
    pub source: Box<TransformSource>,
    pub path: Box<Vec<String>>,
    pub value: Option<Value>,
    pub source_value: Option<Value>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Transform {
    Map(MapTransform),
    Pluck(PluckTransform),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Data {
    String(String),
    Number(i64),
    Bool(bool),
    Map(HashMap<String, Box<Data>>),
    Transform(Transform),
    Array(Vec<Box<Data>>),
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum TransformSource {
    String(String),
    Transform(Transform),
}

impl Transformable for Transform {
    fn get_source(&mut self) -> &mut Box<TransformSource> {
        match *self {
            Transform::Map(ref mut xf) => xf.get_source(),
            Transform::Pluck(ref mut xf) => xf.get_source(),
        }
    }
 
    fn get_source_value(&self) -> Option<&Value> {
        match *self {
            Transform::Map(ref xf) => xf.get_source_value(),
            Transform::Pluck(ref xf) => xf.get_source_value(),
        }
    }

    fn set_source(&mut self, value: Value) -> () {
        match *self {
            Transform::Map(ref mut xf) => xf.set_source(value),
            Transform::Pluck(ref mut xf) => xf.set_source(value),
        }
    }

    fn transform(&mut self, variables: &Map<String, Value>) {
        match *self {
            Transform::Map(ref mut xf) => xf.transform(variables),
            Transform::Pluck(ref mut xf) => xf.transform(variables),
        }
    }
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
            },
            None => {
                println!("attempted to transform {:?} but found None", self.source);
            },
            other => {
                println!("xf_map source resolved to a non-array value {:?}", other);
            }
        }
    }
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
                },
                None => {
                    println!("xf_pluck source resolved to null");
                    None
                },
                other => {
                    println!("xf_pluck source resolved to non object value, {:?}", other);
                    None
                }
            };
        }
    }
}

impl<'de> Deserialize<'de> for MapTransform {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let (xf, src, mapper): (&str, TransformSource, Transform) = Deserialize::deserialize(deserializer)?;

        if xf == "xf_map" {
            Ok(MapTransform::new(src, Box::new(mapper)))
        } else {
            Err(D::Error::custom(format!("tried parsing xf_map, found [{:?}, {:?}, {:?}]", xf, src, mapper)))
        }
    }
}

impl<'de> Deserialize<'de> for PluckTransform {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let (xf, src, path): (&str, TransformSource, Vec<String>) = Deserialize::deserialize(deserializer)?;
        
        if xf == "xf_pluck" {
            Ok(PluckTransform::new(src, Box::new(path)))
        } else {
            Err(D::Error::custom(format!("tried parsing xf_map, found [{:?}, {:?}, {:?}]", xf, src, path)))
        }
    }
}
