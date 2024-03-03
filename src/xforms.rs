pub mod map;
pub mod pluck;

use crate::xforms::map::MapTransform;
use crate::xforms::pluck::PluckTransform;
use serde::Deserialize;
use serde_json::{Map, Value};
use std::ops::DerefMut;

pub trait Transformable {
    fn get_source(&mut self) -> &mut Box<TransformSource>;
    fn get_source_value(&self) -> Option<&Value>;
    fn set_source(&mut self, value: Value) -> ();
    fn transform(&mut self, variables: &Map<String, Value>) -> ();
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Transform {
    Map(MapTransform),
    Pluck(PluckTransform),
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum TransformSource {
    String(String),
    Transform(Transform),
}

pub fn resolve_source(xform: &mut impl Transformable, variables: &Map<String, Value>) -> bool {
    match xform.get_source().deref_mut() {
        TransformSource::String(s) => {
            if let Some(v) = variables.get(&s[1..]) {
                xform.set_source(v.clone()); // TODO: no clone?
                return true;
            }
        }

        TransformSource::Transform(source_xform) => {
            source_xform.transform(variables);
        }
    };

    false
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
