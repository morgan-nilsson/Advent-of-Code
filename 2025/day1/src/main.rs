static INPUT: &str = include_str!("../input.txt");

fn main() {

    let lines = INPUT.lines();

    let mut pointing = 50;
    let mut zero_count = 0;

    for line in lines {

        // parse direction
        let (direction, value) = line.split_at(1);
        let value: i32 = value.trim().parse().unwrap();

        if direction == "L" {
            pointing -= value;
            pointing %= 100;
        }        else if direction == "R" {
            pointing += value;
            pointing %= 100;
        } else {
            panic!("Unknown direction");
        }

        if pointing == 0 {
            zero_count += 1;
        }
    }

    println!("Number of times pointing to 0: {}", zero_count);

}
