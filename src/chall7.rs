use std::fs;

pub fn part1() {
    let content = fs::read_to_string("day7_input.txt").expect("Failed to read file");
    let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
    let mut splitters_met = 0;
    //The way i did it in part2 is way better but I couldn't be bothered in part1 to not use an unsafe block
    unsafe {
        for j in 0..lines[0].len() {
            if lines[0].as_bytes()[j] == b'S' {
                lines[1].as_mut_vec()[j] = b'|';
            }
        }
        for i in 1..lines.len() - 1 {
            println!("Processing line {}: {}", i, lines[i]);
            let line = lines[i].clone();

            let next_line = lines[i + 1].as_mut_vec();
            if line.as_bytes()[0] == b'|' {
                if next_line[0] == b'.' {
                    next_line[0] = b'|';
                } else if next_line[0] == b'^' {
                    next_line[1] = b'|';
                    splitters_met += 1;
                }
            }
            for j in 1..line.len() - 1 {
                if line.as_bytes()[j] == b'|' {
                    if next_line[j] == b'.' {
                        next_line[j] = b'|';
                    } else if next_line[j] == b'^' {
                        next_line[j + 1] = b'|';
                        next_line[j - 1] = b'|';
                        splitters_met += 1;
                    }
                }
            }
            if line.as_bytes()[line.len() - 1] == b'|' {
                if next_line[line.len() - 1] == b'.' {
                    next_line[line.len() - 1] = b'|';
                } else if next_line[line.len() - 1] == b'^' {
                    next_line[line.len() - 2] = b'|';
                    splitters_met += 1;
                }
            }
        }
        println!("Total splitters met: {}", splitters_met);
    }
}

pub fn part2() {
    let content = fs::read_to_string("day7_input.txt").expect("Failed to read file");
    let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
    //Transform lines to grid of ints : 0 for ., S is represented as 1, ^ as -1
    let mut grid: Vec<Vec<i64>> = lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => 0,
                    'S' => 1,
                    '^' => -1,
                    _ => panic!("Unexpected character in input"),
                })
                .collect()
        })
        .collect();
    // The algorithm makes it so each line contains at non -1 positions the number of paths that reach that position -> summing the last lines without the -1 gives us the result.
    for i in 0..grid.len() - 1 {
        for j in 0..grid[i].len() {
            if grid[i][j] > 0 {
                if grid[i + 1][j] == -1 {
                    if j > 0 {
                        grid[i + 1][j - 1] += grid[i][j];
                    }
                    if j < grid[i].len() - 1 {
                        grid[i + 1][j + 1] += grid[i][j];
                    }
                } else {
                    grid[i + 1][j] += grid[i][j]; // 0 OR already-populated pass-through
                }
            }
        }
    }
    let result: i64 = grid[grid.len() - 1].iter().filter(|&&x| x != -1).sum();
    println!("Grid after processing:");
    for row in &grid {
        println!("{:?}", row);
    }
    println!("Result: {}", result);
}
