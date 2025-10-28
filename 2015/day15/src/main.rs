use std::{collections::HashMap, hash::Hash};
#[macro_use] extern crate scan_fmt;

static INPUT: &str = include_str!("../input.txt");

fn main() {
    let mut map: HashMap<String, (i32, i32, i32, i32, i32)> = HashMap::new();
    for line in INPUT.lines() {
        let (name, cap, dur, fla, tex, cal) = scan_fmt!(
            line,
            "{}: capacity {d} , durability {d} , flavor {d} , texture {d} , calories {d}",
            String,
            i32,
            i32,
            i32,
            i32,
            i32
        ).unwrap();
        map.insert(name, (cap, dur, fla, tex, cal));
    }

    let ingredient_names: Vec<String> = map.keys().cloned().collect();
    let mut max_score = 0;

    for a in 0..=100 {
        for b in 0..=(100 - a) {
            for c in 0..=(100 - a - b) {
                let d = 100 - a - b - c;
                let mut amounts: HashMap<String, i32> = HashMap::new();

                amounts.insert(ingredient_names[0].clone(), a);
                amounts.insert(ingredient_names[1].clone(), b);
                amounts.insert(ingredient_names[2].clone(), c);
                amounts.insert(ingredient_names[3].clone(), d);

                let score = calculate_score(&map, &amounts);
                if score > max_score {
                    max_score = score;
                }
            }
        }
    }

    println!("Max score: {}", max_score);


}

fn calculate_score(
    ingredients: &HashMap<String, (i32, i32, i32, i32, i32)>,
    amounts: &HashMap<String, i32>,
) -> i32 {
    let mut total_cap = 0;
    let mut total_dur = 0;
    let mut total_fla = 0;
    let mut total_tex = 0;

    for (name, &amount) in amounts.iter() {
        if let Some(&(cap, dur, fla, tex, _)) = ingredients.get(name) {
            total_cap += cap * amount;
            total_dur += dur * amount;
            total_fla += fla * amount;
            total_tex += tex * amount;
        }
    }

    total_cap = total_cap.max(0);
    total_dur = total_dur.max(0);
    total_fla = total_fla.max(0);
    total_tex = total_tex.max(0);

    total_cap * total_dur * total_fla * total_tex
}