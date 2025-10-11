const INPUTS: &str = "
";

fn main() {
    let nice_count = INPUTS.lines().filter(|line| is_nice(line)).count();
    print!("Number of nice strings: {}\n", nice_count);
}

fn is_nice(s: &str) -> bool {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let forbidden = ["ab", "cd", "pq", "xy"];

    s.chars().filter(|c| vowels.contains(c)).count() >= 3
        && s.chars()
            .zip(s.chars().skip(1))
            .any(|(a, b)| a == b)
        && !forbidden.iter().any(|&f| s.contains(f))
}