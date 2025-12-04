use core::num;

static INPUT: &str = include_str!("../input.txt");

fn main() {

    let mut sum = 0;
    for line in INPUT.lines() {
        let max_jolt = find_max_jolt(line, 12);
        println!("Max jolt for line '{}': {}", line, max_jolt);
        sum += max_jolt;
    }

    println!("Total sum of max jolts: {}", sum);

}

fn find_max_jolt(input: &str, num_of_digits: usize) -> usize {

    let mut max_indexes = vec![0; num_of_digits as usize];
    // select largest digit in the input string but leave at least one character
    let chars = input.chars().collect::<Vec<char>>();

    for i in 0..num_of_digits as usize {
        let max_index_start = if i == 0 { 0 } else { max_indexes[i - 1] + 1 };

        max_indexes[i] = max_index_start;
        for j in (max_index_start)..chars.len() - (num_of_digits as usize - i - 1) {
            if chars[j] > chars[max_indexes[i]] {
                max_indexes[i] = j;
            }
        }

        println!("Selected digit: {} at index {}", chars[max_indexes[i]], max_indexes[i]);
    }

    let mut sum: usize = 0;
    for i in 0..num_of_digits as usize {
        sum += chars[max_indexes[i]].to_digit(10).unwrap() as usize * 10usize.pow((num_of_digits - i - 1) as u32) as usize;
    }

    return sum;
}
