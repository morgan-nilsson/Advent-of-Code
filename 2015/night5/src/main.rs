const INPUTS: &str = include_str!("../input.txt");
//const INPUTS: &str = "aababa
//aabbaba
//aabb
//aba";

fn main() {
    let nice_count = INPUTS.lines().filter(|line| is_nice(line)).count();
    print!("Number of nice strings: {}\n", nice_count);
}

fn is_nice(s: &str) -> bool {

    // It contains a pair of any two letters that appears at least twice in the string without overlapping, like xyxy (xy) or aabcdefgaa (aa), but not like aaa (aa, but it overlaps).

    let has_repeated_pair = {
        let bytes = s.as_bytes();
        let mut pairs = std::collections::HashMap::new();
        let mut found = false;

        for i in 0..bytes.len() - 1 {
            let pair = (bytes[i], bytes[i + 1]);
            if let Some(&first_index) = pairs.get(&pair) {
                if i - first_index > 1 {
                    found = true;
                    break;
                }
            } else {
                pairs.insert(pair, i);
            }
        }

        found
    };

    // It contains at least one letter which repeats with exactly one letter between them, like xyx, abcdefeghi (efe), or even aaa.

    let has_repeat_with_one_between = {
        let bytes = s.as_bytes();
        let mut found = false;

        for i in 0..bytes.len() - 2 {
            if bytes[i] == bytes[i + 2] {
                found = true;
                break;
            }
        }

        found
    };

    has_repeated_pair && has_repeat_with_one_between
}