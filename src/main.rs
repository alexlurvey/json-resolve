use std::collections::HashMap;
use json_resolve::resolve;

fn main() {
    let json: HashMap<&str, &str> = HashMap::from([
        ("one", "ONE"),
    ]);
    resolve(json);
}
