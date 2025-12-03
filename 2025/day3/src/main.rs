static INPUT: &str = include_str!("../input.txt");

fn main() {

    let mut sum = 0;
    for line in INPUT.lines() {
        let max_jolt = find_max_jolt(line);
        sum += max_jolt;
    }

    println!("Total sum of max jolts: {}", sum);

}

fn find_max_jolt(input: &str) -> u32 {
    // select largest digit in the input string but leave at least one character
    let chars = input.chars().collect::<Vec<char>>();

    let mut max_index = 0;
    for i in 0..chars.len() - 1 {
        if chars[i] > chars[max_index] {
            max_index = i;
        }
    }

    let mut second_max_index = max_index + 1;
    for i in (max_index + 1)..chars.len() {
        if chars[i] > chars[second_max_index] {
            second_max_index = i;
        }
    }

    return chars[max_index].to_digit(10).unwrap() * 10 + chars[second_max_index].to_digit(10).unwrap();
}
