use crate::xforms::{TransformSource, Transformable};
use serde::de::{Deserializer, Error};
use serde::Deserialize;
use serde_json::{Map, Value};

#[derive(Debug, PartialEq)]
pub struct PluckTransform {
    pub source: Box<TransformSource>,
    pub path: Vec<String>,
    pub value: Option<Value>,
    pub source_value: Option<Value>,
}

impl PluckTransform {
    pub fn new(source: TransformSource, path: Vec<String>) -> Self {
        PluckTransform {
            source: Box::new(source),
            path,
            value: None,
            source_value: None,
        }
    }
}

impl Transformable for PluckTransform {
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
        let found = self.resolve_source(variables);
        if found {
            if let Some(Value::Object(obj)) = self.get_source_value() {
                let plucked = pluck(obj, &self.path);
                if let Some(value) = plucked {
                    self.value = Some(value.clone());
                } else {
                    // println!(
                    //     "xf_pluck path could not resolve to a value, {:?}",
                    //     self.path
                    // );
                }
            } else {
                // println!("xf_pluck source resolved to non-object value");
            }
        }
    }
}

pub fn pluck<'a>(source: &'a Map<String, Value>, path: &[String]) -> Option<&'a Value> {
    let mut result: Option<&Value> = None;
    let mut lookup: &Map<String, Value> = source;

    for key in path.iter() {
        if let Some(value) = lookup.get(key) {
            result = Some(value);
            if let Value::Object(obj) = value {
                lookup = &obj;
            }
        } else {
            return None;
        }
    }

    result
}

impl<'de> Deserialize<'de> for PluckTransform {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let (xf, src, path): (&str, TransformSource, Vec<String>) =
            Deserialize::deserialize(deserializer)?;

        if xf == "xf_pluck" {
            Ok(PluckTransform::new(src, path))
        } else {
            Err(D::Error::custom(format!(
                "tried parsing xf_pluck, found [{:?}, {:?}, {:?}]",
                xf, src, path
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use serde_json::{Map, Value};

    use crate::resolve;
    use crate::xforms::Transform;
    use crate::Data;

    #[test]
    fn xf_pluck_basic_functionality() {
        let json = r#"
            { "data": ["xf_pluck", "$var", ["prop"]] }
        "#;

        let variables: Map<String, Value> = json!({
            "var": { "prop": "testing" }
        })
        .as_object()
        .unwrap()
        .to_owned();

        let result = resolve(json, &variables);

        let result = result.get("data").unwrap();

        if let Data::Transform(Transform::Pluck(xf)) = result {
            let result = xf
                .value
                .as_ref()
                .expect("value of PluckTransform at 'data' was not resolved")
                .as_str()
                .expect("value of PluckTransform at 'data' was not resolved to a string slice");

            assert_eq!(result, "testing");
        } else {
            panic!("'data' was not serialized into a PluckTransform");
        }
    }
}
