use std::collections::HashMap;

#[macro_use] extern crate scan_fmt;

const MFCSAM: (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = (3, 7, 2, 3, 0, 0, 5, 3, 2, 1);

static INPUT: &str = include_str!("../input.txt");

fn main() {
    for line in INPUT.lines() {
        let mut map: HashMap<&str, i32> = HashMap::new();
        let (number, prop1, val1, prop2, val2, prop3, val3) = scan_fmt!(
            line,
            "Sue {}: {}: {}, {}: {}, {}: {}",
            u32,
            String,
            i32,
            String,
            i32,
            String,
            i32
        ).unwrap();
        map.insert(&prop1, val1);
        map.insert(&prop2, val2);
        map.insert(&prop3, val3);

        if is_match(&map) {
            println!("{:?}", map);
            println!("{}", line);
            println!("sue: {}", number);
        }
    }
}

fn is_match(map: &HashMap<&str, i32>) -> bool {
    for (key, &val) in map.iter() {
        match *key {
            "children" => {
                if val != MFCSAM.0 {
                    return false;
                }
            }
            "cats" => {
                if val != MFCSAM.1 {
                    return false;
                }
            }
            "samoyeds" => {
                if val != MFCSAM.2 {
                    return false;
                }
            }
            "pomeranians" => {
                if val != MFCSAM.3 {
                    return false;
                }
            }
            "akitas" => {
                if val != MFCSAM.4 {
                    return false;
                }
            }
            "vizslas" => {
                if val != MFCSAM.5 {
                    return false;
                }
            }
            "goldfish" => {
                if val != MFCSAM.6 {
                    return false;
                }
            }
            "trees" => {
                if val != MFCSAM.7 {
                    return false;
                }
            }
            "cars" => {
                if val != MFCSAM.8 {
                    return false;
                }
            }

            "perfumes" => {
                if val != MFCSAM.9 {
                    return false;
                }
            }
            _ => {
                panic!("Unknown property: {}", key);
            }
        }
    }

    return true;
}
