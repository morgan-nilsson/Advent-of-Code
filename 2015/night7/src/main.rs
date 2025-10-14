use std::collections::{HashMap, VecDeque};

static INPUT: &str = include_str!("../input.txt");

fn main() {

    let mut vars: HashMap<&str, u16> = HashMap::new();
    let mut waitlist: VecDeque<&str> = VecDeque::new();

    let lines = INPUT.lines();
    waitlist.push_back("956 -> b");

    for line in lines {
        waitlist.push_back(line);
    }


    while !waitlist.is_empty() {
        let line = waitlist.pop_front().unwrap();

        let parts: Vec<&str> = line.split(" ").collect();

        // assignment
        if parts[1] == "->" {

            let value = match parts[0].parse::<u16>() {
                Ok(v) => { Some(v) }
                Err(_) => {
                    match vars.get(parts[0]) {
                        Some(v) => { Some(*v) }   
                        None => { None }
                    }
                }
            };
            if value.is_none() {
                waitlist.push_back(&line);
                continue;
            }

            vars.insert(parts[2], value.unwrap());
        } else if parts[0] == "NOT" && parts[2] == "->" {

            let value = match parts[1].parse::<u16>() {
                Ok(v) => { Some(v) }
                Err(_) => {
                    match vars.get(parts[1]) {
                        Some(v) => { Some(*v) }   
                        None => { None }
                    }
                }
            };
            if value.is_none() {
                waitlist.push_back(&line);
                continue;
            }

            vars.insert(parts[3], !(value.unwrap()));

        } else {
            let variable1 = match parts[0].parse::<u16>() {
                Ok(v) => { Some(v) }
                Err(_) => {
                    match vars.get(parts[0]) {
                        Some(v) => { Some(*v) }   
                        None => { None }
                    }
                }
            };
            if variable1.is_none() {
                waitlist.push_back(&line);
                continue;
            }
            let variable1 = variable1.unwrap();

            let variable2 = match parts[2].parse::<u16>() {
                Ok(v) => { Some(v) }
                Err(_) => {
                    match vars.get(parts[2]) {
                        Some(v) => { Some(*v) }   
                        None => { None }
                    }
                }
            };
            if variable2.is_none() {
                waitlist.push_back(&line);
                continue;
            }
            let variable2 = variable2.unwrap();

            let destination_variable = parts[4];

            match parts[1] {

                "AND" => {
                    vars.insert(destination_variable, variable1 & variable2);
                }
                "LSHIFT" => {
                    vars.insert(destination_variable, variable1 << variable2);
                }
                "OR" => {
                    vars.insert(destination_variable, variable1 | variable2);
                }
                "RSHIFT" => {
                    vars.insert(destination_variable, variable1 >> variable2);
                }
                _ => {
                    panic!()
                }

            }
        }

    }

    for e in &vars {
        let (c, v) = e;

        println!("{} = {}", c, v);
    }

    println!("a = {}", vars.get("a").unwrap_or(&0))
}