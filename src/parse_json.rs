use serde::Deserialize;
use serde::de::{Deserializer, Error};
use std::collections::HashMap;
use std::ops::Deref;
use serde_json::{Map, Value};

//fn resolve_source(xform: &mut Transform, variables: &Map<String, Value>) -> bool {
//
//    let key = match xform.source.deref() {
//        TransformSource::String(s) => &s[1..],
//        TransformSource::Transform(xform) => {
//            panic!("unknown transform source for {:?}", xform)
//        },
//    };
//    println!("resovle source: {:?} -> {}", xform.source.deref(), key);
//    if let Some(v) = variables.get(key) {
//        xform.value = Some(v.to_owned());
//        return true
//    }
//
//    false
//}

#[derive(Debug, PartialEq)]
pub struct MapTransform {
    pub source: Box<TransformSource>,
    pub mapper: Box<Transform>,
    pub value: Option<Value>,
}

#[derive(Debug, PartialEq)]
pub struct PluckTransform {
    pub source: Box<TransformSource>,
    pub path: Box<Vec<String>>,
    pub value: Option<Value>,
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

impl MapTransform {
    pub fn new(source: TransformSource, mapper: Box<Transform>) -> Self {
        MapTransform {
            source: Box::new(source),
            mapper,
            value: None,
        }
    }

    pub fn resolve_source(&mut self, _variables: &Map<String, Value>) -> bool {
        let key = match self.source.deref() {
            TransformSource::String(s) => &s[1..],
            TransformSource::Transform(_xform) => {
                println!("TOOD: resolve transform for another transform's source");
                ""
            },
        };
        println!("resovle source: {:?} -> {}", self.source.deref(), key);
//        if let Some(v) = variables.get(key) {
//            self.value = Some(v.to_owned());
//            return true
//        }

        false
    }
}

impl PluckTransform {
    pub fn new(source: TransformSource, path: Box<Vec<String>>) -> Self {
        PluckTransform {
            source: Box::new(source),
            path,
            value: None,
        }
    }

    pub fn resolve_source(&mut self, _variables: &Map<String, Value>) -> bool {
        let key = match self.source.deref() {
            TransformSource::String(s) => &s[1..],
            TransformSource::Transform(_xform) => {
                println!("TOOD: resolve transform for another transform's source");
                ""
            },
        };
        println!("resovle source: {:?} -> {}", self.source.deref(), key);
//        if let Some(v) = variables.get(key) {
//            self.value = Some(v.to_owned());
//            return true
//        }

        false
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
