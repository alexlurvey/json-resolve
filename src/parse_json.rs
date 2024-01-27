use serde::Deserialize;
use serde::de::{Deserializer, Error};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct MapTransform(String, pub String, pub Vec<String>);

#[derive(Debug, PartialEq)]
pub struct PluckTransform(String, pub String, pub Vec<String>);

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Transform {
    MapTransform(MapTransform),
    PluckTransform(PluckTransform),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Data {
    String(String),
    Number(i64),
    Bool(bool),
    Map(HashMap<String, Box<Data>>),
    Tranform(Transform),
    Array(Vec<Box<Data>>),
}

impl MapTransform {
    pub fn new(source: &str, list: Vec<String>) -> Self {
        MapTransform(String::from("xf_map"), String::from(source), list)
    }
}

impl PluckTransform {
    pub fn new(source: &str, list: Vec<String>) -> Self {
        PluckTransform(String::from("xf_pluck"), String::from(source), list)
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
