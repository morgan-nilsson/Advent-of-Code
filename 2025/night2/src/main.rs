static INPUT: &str = include_str!("../input.txt");

#[macro_use] extern crate scan_fmt;

fn main() {
    // 11-22,95-115,

    let mut sum: u64 = 0;

    let ranges = parse(INPUT.lines().next().unwrap());
    for (min, max) in ranges {

        for value in min..=max {
            if is_invalid(value) {
                println!("invalid: {}", value);
                sum += value;
            }
        }

    }

    println!("{}", sum);
}

fn is_invalid(value: u64) -> bool {
    let value = value.to_string();

    // numbers have leading zeroes
    if value.starts_with('0') {
        return true;
    }

    // sequence of digits repeated
    for i in 0..value.len()-1 / 2 {
        // must split evenly into i sized chunks
        if value.len() % (i + 1) != 0 {
            continue;
        }

        let slice = &value[0..i+1];
        //println!("Checking slice: {} of {}", slice, value);
        for j in (i+1..value.len()).step_by(slice.len()) {
            let next_slice = &value[j..j + slice.len()];
            if slice != next_slice {
                break;
            }

            if j + i + 1 == value.len() {
                return true;
            }
        }

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
