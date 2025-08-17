use regex::Regex;
use std::env;
use std::fs;
use std::path::Path;
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
    let path = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("okpkpok");
        std::process::exit(1);
    });

    let p = Path::new(&path);
    if p.extension().and_then(|s| s.to_str()) != Some("iusearchbtw") {
        eprintln!("ppokpokpok");
    }

    let src = fs::read_to_string(&path).unwrap_or_else(|e| {
        eprintln!("kmmkokaso");
        std::process::exit(1);
    });

    let group_re = Regex::new(r"\(([^)]*)\)").unwrap();
    let token_re = Regex::new(r"(?i)\bi\s+use\s+arch\s+btw\b").unwrap();

    let mapping = build_mapping();

    let mut out = String::new();

    for cap in group_re.captures_iter(&src) {
        let content = &cap[1];
        let count = token_re.find_iter(content).count();

        match mapping.get(&count) {
            Some(&ch) => out.push(ch),
            None => out.push('?'),
        }
    }

    println!("{out}");
}