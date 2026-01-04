static INPUT: &str = include_str!("../input.txt");

fn main() {

    let mut ranges: Vec<(u64, u64)> = INPUT
        .lines()
        .filter_map(|line| {
            let mut parts = line.split('-');
            if let (Some(start), Some(end)) = (parts.next(), parts.next()) {
                if let (Ok(start), Ok(end)) = (start.parse::<u64>(), end.parse::<u64>()) {
                    return Some((start, end));
                }
            }
            None
        })
        .collect();

    // sort the ranges by ascending start value
    ranges.sort_by_key(|&(start, _)| start);

    let mut merged_ranges: Vec<(u64, u64)> = Vec::new();

    // reduce the set of ranges
    for (start, end) in ranges {
        if let Some((_, last_end)) = merged_ranges.last_mut() {
            if start <= *last_end + 1 {
                // Ranges overlap or are contiguous, merge them
                *last_end = (*last_end).max(end);
            } else {
                merged_ranges.push((start, end));
            }
        } else {
            merged_ranges.push((start, end));
        }
    }

    let mut total_values = 0;
    for (start, end) in merged_ranges {
        total_values += end - start + 1;
    }
    println!("Total values in range: {}", total_values);
}
