use std::fs;

fn neighbors(x: usize, y: usize, width: usize, height: usize) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    if x > 0 {
        result.push((x - 1, y));
    }
    if x < width - 1 {
        result.push((x + 1, y));
    }
    if y > 0 {
        result.push((x, y - 1));
    }
    if y < height - 1 {
        result.push((x, y + 1));
    }
    if x > 0 && y > 0 {
        result.push((x - 1, y - 1));
    }
    if x < width - 1 && y > 0 {
        result.push((x + 1, y - 1));
    }
    if x > 0 && y < height - 1 {
        result.push((x - 1, y + 1));
    }
    if x < width - 1 && y < height - 1 {
        result.push((x + 1, y + 1));
    }
    result
}
pub fn part1() {
    let contents =
        fs::read_to_string("day4_input.txt").expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.lines().collect();
    let grid: Vec<Vec<char>> = lines
        .iter()
        .map(|line| line.trim().chars().collect())
        .collect();
    let width = grid[0].len();
    let height = grid.len();
    let mut removable = 0;
    for x in 0..width {
        for y in 0..height {
            let cell = grid[y][x];
            if cell != '@' {
                continue;
            }
            let neighbor_coords = neighbors(x, y, width, height);
            let mut neighbor_rolls = 0;
            for (nx, ny) in neighbor_coords {
                let neighbor_cell = grid[ny][nx];
                if neighbor_cell == cell {
                    neighbor_rolls += 1;
                }
            }
            if neighbor_rolls < 4 {
                removable += 1;
            }
        }
    }
    println!("Removable cells: {}", removable);
}

pub fn part2() {
    let contents =
        fs::read_to_string("day4_input.txt").expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.lines().collect();
    let grid: Vec<Vec<char>> = lines
        .iter()
        .map(|line| line.trim().chars().collect())
        .collect();
    let mut grid = grid; // Make grid mutable
    let width = grid[0].len();
    let height = grid.len();
    let mut removable = 0;
    let mut flag = true;
    while flag {
        flag=false;
        for x in 0..width {
            for y in 0..height {

                let cell = grid[y][x];
                if cell != '@' {
                    continue;
                }
                let neighbor_coords = neighbors(x, y, width, height);
                let mut neighbor_rolls = 0;
                for (nx, ny) in neighbor_coords {
                    let neighbor_cell = grid[ny][nx];
                    if neighbor_cell == cell {
                        neighbor_rolls += 1;
                    }
                }
                if neighbor_rolls < 4 {
                    removable += 1;
                    grid[y][x] = '.'; // Mark the cell as removed
                    flag=true;
                }
            }
        }
    }

    println!("Removable cells: {}", removable);
}
