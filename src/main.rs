use std::collections::HashMap;

fn build_mapping() -> HashMap<usize, char> {
    let mut map = HashMap::new();

    for (i, c) in ('A'..='Z').enumerate() {
        map.insert(i + 1, c);
    }

    for (i, c) in ('a'..='z').enumerate() {
        map.insert(i + 27, c);
    }

    map.insert(53, '.');
    map.insert(54, '-');
    map.insert(55, '_');
    map.insert(56, '(');
    map.insert(57, ')');
    map.insert(58, '{');
    map.insert(59, '}');
    map.insert(60, '[');
    map.insert(61, ']');

    map
}

fn main() {

}
