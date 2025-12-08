static INPUT: &str = include_str!("../input.txt");

fn main() {

    let mut grid: Vec<Vec<bool>> = Vec::new();

    for line in INPUT.lines() {

        grid.push(Vec::new());
        let row = grid.len() - 1;

        for c in line.chars() {

            if c == '@' {

                grid[row].push(true);
            } else {

                grid[row].push(false);
            }
        }
    }

    let mut removed = 0;

    let mut removed_in_last_iteration = true;

    while removed_in_last_iteration == true {

        removed_in_last_iteration = false;

        for y in 0..grid.len() {

            for x in 0..grid[y].len() {

                if grid[y][x] == true {

                    let neighbor_count = get_neighbor_count(&grid, x, y);

                    if neighbor_count < 4 {

                        grid[y][x] = false;
                        removed += 1;
                        removed_in_last_iteration = true;
                    }
                }
            }
        }
    }

    println!("Removed: {}", removed);

}


fn get_neighbor_count(grid: &Vec<Vec<bool>>, x: usize, y: usize) -> usize {

    let mut count = 0;

    let directions: [(isize, isize); 8] = [
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];

    for (dx, dy) in directions.iter() {

        let nx = x as isize + dx;
        let ny = y as isize + dy;

        if nx >= 0 && ny >= 0 {

            let nx = nx as usize;
            let ny = ny as usize;

            if ny < grid.len() && nx < grid[ny].len() {

                if grid[ny][nx] == true {

                    count += 1;

                }
            }
        }
    }

    count
}