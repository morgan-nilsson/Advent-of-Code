static INPUT: &str = include_str!("../input.txt");

#[macro_use] extern crate scan_fmt;

fn main() {
    // 11-22,95-115,

    let mut sum: u64 = 0;

    let ranges = parse(INPUT.lines().next().unwrap());
    for (min, max) in ranges {

        for value in min..=max {
            if is_invalid(value) {
                sum += value;
            }
        }

    }

    println!("{}", sum);
}

fn is_invalid(value: u64) -> bool {
    let value = value.to_string();
    // sequence of digits repeated twice.
    let first = &value[0..value.len()/2];
    let second = &value[value.len()/2..];
    if first == second {
        return true;
    }

    // numbers have leading zeroes
    if value.starts_with('0') {
        println!("leading zero: {}", value);
        return true;
    }

    false
}

fn parse(line: &str) -> Vec<(u64, u64)>  {
    let mut ranges = Vec::new();
    let mut slice_start = 0;
    loop {
        let result = scan_fmt!(&line[slice_start..], "{}-{},", u64, u64);
        if result.is_err() {
            break;
        }

        let (min, max) = result.unwrap();
        ranges.push((min, max));
        let res = line[slice_start..].find(',');
        if let Some(pos) = res {
            slice_start += pos + 1;
        } else {
            break;
        }
    }

    ranges
}
