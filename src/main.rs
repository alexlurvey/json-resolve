use serde_json::json;
use serde_json::{Map, Value};
use json_resolve::resolve;

const JSON: &'static str = r#"
    {
        "obj": {
            "nest": "string",
            "map": ["xf_map", "$src", ["lookup", "my", "data"]],
            "pluck": ["xf_pluck", "$data", ["pluck", "me", "daddy"]],
            "bool": true,
            "num": 42,
            "more_nest": {
                "mapper": ["xf_pluck", "$data", ["not_found_prop"]]
            }
        },
        "array": [
            1,
            "one",
            true,
            { "obj": "testing" },
            [1, 2, 3],
            ["xf_map", "$src", ["prop"]],
            ["xf_pluck", "$plk", ["plk_prop"]]
        ],
        "num": -98,
        "string": "testing",
        "bool": false,
        "map": ["xf_map", "$src", ["map_property"]],
        "pluck": ["xf_pluck", "$not_found_object", ["pluck_property"]]
    }"#;

fn main() {
    let variables: Map<String, Value> = json!({
        "data": {
            "lookup": {
                "my": { "data": "my_data" },
            }
        },
        "source": ["one", "two", "three"]
    }).as_object().unwrap().to_owned();

    resolve(JSON, &variables);
}
