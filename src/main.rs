use json_resolve::resolve;

const JSON: &'static str = r#"
    {
        "obj": {
            "nest": "string",
            "map": ["xf_map", "$data", ["lookup", "my", "data"]],
            "pluck": ["xf_pluck", "$data", ["pluck", "me", "daddy"]],
            "bool": true,
            "num": 42,
            "more_nest": {
                "mapper": ["xf_map", "$data", ["prop"]]
            }
        },
        "num": -98,
        "string": "testing",
        "bool": false,
        "map": ["xf_map", "$source", ["map_property"]],
        "pluck": ["xf_pluck", "$object", ["pluck_property"]]
    }"#;

fn main() {
    resolve(JSON);
}
