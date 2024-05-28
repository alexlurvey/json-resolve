use json_resolve::__resolve;
use serde_json::json;
use serde_json::{Map, Value};

const JSON: &str = r#"
    {
        "obj": {
            "nest": "string",
            "map": ["xf_map", "$src", ["xf_pluck", "$", ["lookup", "my", "data"]]],
            "pluck": ["xf_pluck", "$data", ["lookup", "my", "data"]],
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
            ["xf_map", "$src", ["xf_pluck", "$", ["prop"]]],
            ["xf_pluck", "$plk", ["plk_prop"]]
        ],
        "num": -98,
        "string": "testing",
        "bool": false,
        "map": ["xf_map", "$src", ["xf_pluck", "$", ["map_property"]]],
        "pluck": ["xf_pluck", "$not_found_object", ["pluck_property"]],
        "transform_as_source": ["xf_map", ["xf_pluck", "$data", ["nested_array"]], ["xf_pluck", "$", ["prop"]]]
    }"#;

fn main() {
    let variables: Map<String, Value> = json!({
        "data": {
            "lookup": {
                "my": { "data": "my_data" },
            },
            "nested_array": [
                { "prop": 1 },
                { "prop": 2 },
            ],
        },
        "source": ["one", "two", "three"]
    })
    .as_object()
    .unwrap()
    .to_owned();

    let result = __resolve(JSON, &variables);

    println!("result -- {:?}", result);
}
