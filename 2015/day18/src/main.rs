use std::vec;

static INPUT: &str = include_str!("../input.txt");

fn main() {
    let mut state: Vec<Vec<bool>> = vec![vec![]];

    for line in INPUT.lines() {
        state.push(
            line.chars()
                .map(|c| match c {
                    '#' => true,
                    '.' => false,
                    _ => panic!("Unexpected character in input"),
                })
                .collect(),
        );
    }

    // Remove the initial empty vector
    state.remove(0);

    const STEPS: usize = 100;

    for _ in 0..STEPS {
        let mut new_state = state.clone();

        for x in 0..state.len() {
            for y in 0..state[0].len() {
                let neighbors = count_neighbors(&state, x, y);

                if state[x][y] {
                    // Current cell is alive
                    if neighbors < 2 || neighbors > 3 {
                        new_state[x][y] = false; // Cell dies
                    }
                } else {
                    // Current cell is dead
                    if neighbors == 3 {
                        new_state[x][y] = true; // Cell becomes alive
                    }
                }
            }
        }

        state = new_state;
    }

    let alive_count: usize = state
        .iter()
        .map(|row| row.iter().filter(|&&cell| cell).count())
        .sum();

    println!("Number of alive {}", alive_count);
}

fn count_neighbors(state: &Vec<Vec<bool>>, x: usize, y: usize) -> usize {
    let mut count = 0;
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for (dx, dy) in directions.iter() {
        let nx = x as isize + dx;
        let ny = y as isize + dy;

        if nx >= 0
            && ny >= 0
            && (nx as usize) < state.len()
            && (ny as usize) < state[0].len()
            && state[nx as usize][ny as usize]
        {
            count += 1;
        }
    }

    count
}