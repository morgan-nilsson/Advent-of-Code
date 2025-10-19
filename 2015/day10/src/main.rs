use std::fmt::Write;

fn main() {
    let input = "1113122113";
    let iterations = 40;
    let mut curr = input.to_string();

    for _ in 0..iterations {
        let mut next = String::new();
        let mut chars = curr.chars();
        let mut last = chars.next().unwrap();
        let mut count = 1;

        for ch in chars {
            if ch == last {
                count += 1;
            } else {
                write!(next, "{}{}", count, last).unwrap();
                last = ch;
                count = 1;
            }
        }

        write!(next, "{}{}", count, last).unwrap();
        curr = next;
    }

    println!("Length: {}", curr.len());
}
