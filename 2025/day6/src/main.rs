static INPUT: &str = include_str!("../input.txt");

fn main() {
    let ops = INPUT.lines().last().unwrap().split_whitespace().collect::<Vec<&str>>();
    let mut column_sums: Vec<u64> = Vec::new();

    let mut lines_itr = INPUT.lines();
    let initial_line = lines_itr.next().unwrap().trim();
    for val in initial_line.split_whitespace() {
        let value: u64 = val.trim().parse().unwrap();
        column_sums.push(value);
    }

    for line in lines_itr {
        if line.is_empty() || line.contains('+') || line.contains('*') {
            break;
        }
        for (i, val) in line.split_whitespace().enumerate() {
            println!("Processing column {}: current sum = {}, operation = {}, value = {}", i, column_sums[i], ops[i], val);
            let value: u64 = val.parse().unwrap();
            let new_val = match ops[i] {
                "+" => column_sums[i] + value,
                "*" => column_sums[i] * value,
                _ => panic!("Unsupported operation"),
            };
            column_sums[i] = new_val;
        }
    }

    let mut sum: u64 = 0;
    for col_sum in column_sums {
        print!("{} ", col_sum);
        sum += col_sum;
    }

    println!("\nTotal sum: {}", sum);
}
