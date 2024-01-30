use serde::Deserialize;
use serde::de::{Deserializer, Error};
use std::collections::HashMap;
use serde_json::value::Value;

#[derive(Debug, PartialEq)]
pub struct MapTransform {
    pub source: String,
    pub path: Vec<String>,
    pub value: serde_json::Value,
}

#[derive(Debug, PartialEq)]
pub struct PluckTransform {
    pub source: String,
    pub path: Vec<String>,
    pub value: serde_json::Value,
}

#[derive(Debug, Deserialize)]
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

impl MapTransform {
    pub fn new(source: &str, list: Vec<String>) -> Self {
        MapTransform {
            source: String::from(source),
            path: list,
            value: Value::Null,
        }
    }
}

impl PluckTransform {
    pub fn new(source: &str, list: Vec<String>) -> Self {
        PluckTransform {
            source: String::from(source),
            path: list,
            value: Value::Null,
        }
    }
}

impl<'de> Deserialize<'de> for MapTransform {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let (xf, src, props): (&str, &str, Vec<String>) = Deserialize::deserialize(deserializer)?;

        if xf == "xf_map" {
            Ok(MapTransform::new(src, props))
        } else {
            Err(D::Error::custom(format!("tried parsing xf_map, found [{:?}, {:?}, {:?}]", xf, src, props)))
        }
    }
}

impl<'de> Deserialize<'de> for PluckTransform {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let (xf, src, props): (&str, &str, Vec<String>) = Deserialize::deserialize(deserializer)?;
        
        if xf == "xf_pluck" {
            Ok(PluckTransform::new(src, props))
        } else {
            Err(D::Error::custom(format!("tried parsing xf_map, found [{:?}, {:?}, {:?}]", xf, src, props)))
        }
    }
}
