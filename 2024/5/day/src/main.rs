use std::{collections::HashMap};

fn main() {

    // read file
    const FILE_PATH: &str = "real.txt";

    let contents = std::fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    let rules_and_data: Vec<&str> = contents.split("\n\n").collect();

    let rules = rules_and_data[0];
    let test_cases = rules_and_data[1];

    // none of the values can be after the vec
    let mut rules_map: HashMap<u32, Vec<u32>> = HashMap::new();
    let rules_lines: Vec<&str> = rules.split("\n").collect();
    for line in rules_lines {
        let parts: Vec<&str> = line.split("|").collect();
        let first: u32 = parts[0].trim().parse().expect("not a number");
        let after: u32 = parts[1].trim().parse().expect("not a number");
        rules_map.entry(after).and_modify(|e| {
            e.push(first);
        }).or_insert(vec![first]);
    }

    let test_cases: Vec<Vec<u32>> = test_cases.split("\n").map(
        |s| s.split(",").map(
            |s| s.parse().expect(format!("test is a number, {}\n", s).as_str())
        ).collect()
    ).collect();

    let mut sum = 0;
    for pages in test_cases {
        if is_good(&pages, &rules_map) {
            sum += pages[pages.len() / 2];
            println!("Good");
        } else {
            println!("Bad");
        }
    }
    println!("Sum: {}", sum);
}

fn is_good(pages: &Vec<u32>, rules_map: &HashMap<u32, Vec<u32>>) -> bool {
    println!("Checking pages: {:?}", pages);
    for i in 0..pages.len() {
        let page = pages[i];
        if let Some(requires) = rules_map.get(&page) {
            for req in requires {
                if pages[i+1..pages.len()].contains(req) {
                    // println!("Page {} requires page {}", page, req);
                    return false;
                }
            }
        }
    }
    return true;
}