use std::collections::HashMap;

fn build_mapping() -> HashMap<usize, char> {
    let mut map = HashMap::new();

    for (i, c) in ('A'..='Z').enumerate() {
        map.insert(i + 1, c);
    }

    map
}

fn main() {}
