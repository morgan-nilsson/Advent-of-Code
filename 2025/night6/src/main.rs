use core::num;

static INPUT: &str = include_str!("../input.txt");

fn main() {

    let input: Vec<&str> = INPUT.lines().collect();
    let max_input_line_length = input.iter().map(|line| line.len()).max().unwrap_or(0);

    let mut numbers: Vec<String> = Vec::new();
    numbers.resize(max_input_line_length, String::new());

    for i in 0..(input.len() - 1) {
        let line = input[i];
        for (j, c) in line.chars().enumerate() {
            if c != ' ' {
                numbers[j].push(c);
            }
        }
    }

    let ops = INPUT.lines().last().unwrap().split_whitespace().collect::<Vec<&str>>();

    let mut sum: u64 = 0;
    let mut index = 0;
    for op in ops {

        while numbers[index].is_empty() {
            index += 1;
        }

        let mut local_total = 0;
        while !numbers[index].is_empty() {
            let next_number = numbers[index].parse::<u64>().unwrap();
            println!("Next number: {}", next_number);
            if local_total == 0 {
                local_total = next_number;
            } else {
                local_total = match op {
                    "+" => local_total + next_number,
                    "*" => local_total * next_number,
                    _ => panic!("Unknown operation"),
                }
            }
            index += 1;
            if index >= numbers.len() {
                break;
            }
        }
        //println!("{}", local_total);
        sum += local_total;
    }

    println!("Final sum: {}", sum);

}
