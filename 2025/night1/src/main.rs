static INPUT: &str = include_str!("../input.txt");

fn main() {
    let mut pos: i32 = 50;
    let mut zero_count: usize = 0;

    for line in INPUT.lines().map(str::trim).filter(|l| !l.is_empty()) {
        let dir = &line[0..1];
        let steps: i32 = line[1..].parse().expect("invalid number");

        for _ in 0..steps {
            if dir == "R" {
                pos = (pos + 1) % 100;
            } else if dir == "L" {
                pos = (pos - 1).rem_euclid(100);
            } else {
                panic!("Unknown direction {}", dir);
            }

            if pos == 0 {
                zero_count += 1;
            }
        }
    }

    println!("Total zero hits: {}", zero_count);
}
