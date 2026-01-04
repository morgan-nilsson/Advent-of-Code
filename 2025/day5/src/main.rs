static INPUT: &str = include_str!("../input.txt");

fn main() {

    let mut ranges: Vec<(u64, u64)> = Vec::new();

    for line in INPUT.lines() {

        if line.contains('-') {
            let parts: Vec<&str> = line.split('-').collect();
            let start: u64 = parts[0].parse().unwrap();
            let end: u64 = parts[1].parse().unwrap();
            ranges.push((start, end));
        }

    }

    for i in 0..ranges.len() {
        for j in 0..ranges.len() {
            // remove enveloped ranges
            if i != j {
                if ranges[i].0 <= ranges[j].0 && ranges[i].1 >= ranges[j].1 {
                    ranges[j] = (0, 0);
                }
            }
            // merge overlapping ranges
            if i != j {
                if ranges[i].0 <= ranges[j].1 || ranges[j].0 <= ranges[i].1 {
                    let new_start = std::cmp::min(ranges[i].0, ranges[j].0);
                    let new_end = std::cmp::max(ranges[i].1, ranges[j].1);
                    ranges[i] = (new_start, new_end);
                    ranges[j] = (0, 0);
                }
            }
        }
    }

    for &(start, end) in &ranges {
        println!("Range: {}-{}", start, end);
    }

}
