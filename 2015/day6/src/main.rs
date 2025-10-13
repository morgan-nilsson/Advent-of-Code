use std::io::BufRead;

const INPUT: &str = include_str!("../input.txt");

fn main() {

    let mut grid = vec![vec![false; 1000]; 1000];

    let file_lines = INPUT.lines();

    for line in file_lines {
        let parts: Vec<&str> = line.split(" ").collect();

        let instruction = parts[0];

        let (start_x, start_y) = {
            let coords: Vec<&str> = parts[1].split(",").collect();
            (coords[0].parse::<usize>().unwrap(), coords[1].parse::<usize>().unwrap())
        };
        let (end_x, end_y) = {
            let coords: Vec<&str> = parts[2].split(",").collect();
            (coords[0].parse::<usize>().unwrap(), coords[1].parse::<usize>().unwrap())
        };

        match instruction {
            "on" => {
                for x in start_x..=end_x {
                    for y in start_y..=end_y {
                        grid[x][y] = true;
                    }
                }
            }
            "off" => {
                for x in start_x..=end_x {
                    for y in start_y..=end_y {
                        grid[x][y] = false;
                    }
                }
            }
            "toggle" => {
                for x in start_x..=end_x {
                    for y in start_y..=end_y {
                        grid[x][y] = !grid[x][y];
                    }
                }
            }
            _ => panic!("Unknown instruction"),
        }
    }

    let mut count = 0;
    for row in &grid {
        for &light in row {
            if light {
                count += 1;
            } 
        }
    }

    println!("Number of lights on: {}", count);
}
