static INPUT: &str = include_str!("../input.txt");

fn main() {
    let mut start_of_numeric: isize = -1;
    let input: Vec<char> = INPUT.chars().into_iter().collect();
    let mut sum = 0;
    for i in 0..input.len() {
        let input_char = input.get(i).unwrap();
        let is_alpha = input_char.is_numeric() || *input_char == '-';
        if is_alpha && start_of_numeric == -1 {
            start_of_numeric = i as isize;
        } else if is_alpha == false && start_of_numeric != -1 {
            // collect start_of_numeric..i in a string
            println!(
                "Found numeric from {} to {}: {}",
                start_of_numeric,
                i,
                input
                    .get(start_of_numeric as usize..i)
                    .unwrap()
                    .iter()
                    .collect::<String>()
            );
            let num_str: String = input
                .get(start_of_numeric as usize..i)
                .unwrap()
                .iter()
                .collect();

            let num: i32 = num_str.parse().unwrap();
            sum += num;
            start_of_numeric = -1;
        }
    }

    println!("{}", sum);
}
