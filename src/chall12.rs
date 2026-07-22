use std::{collections::HashSet, fs};

fn rotate_90(shape: &[[u64; 3]; 3]) -> [[u64; 3]; 3] {
    let mut rotated = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            rotated[j][2 - i] = shape[i][j];
        }
    }
    rotated
}

fn flip_horizontal(shape: &[[u64; 3]; 3]) -> [[u64; 3]; 3] {
    let mut flipped = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            flipped[i][2 - j] = shape[i][j];
        }
    }
    flipped
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Variation {
    cells: Vec<(usize, usize)>,
}

fn can_place(grid: &Vec<Vec<u64>>, var: &Variation, x: usize, y: usize) -> bool {
    for &(r, c) in &var.cells {
        if grid[x + r][y + c] == 1 {
            return false;
        }
    }
    true
}

fn backtrack(
    grid: &mut Vec<Vec<u64>>,
    shapes: &Vec<Vec<Variation>>,
    gift_numbers: &mut Vec<u64>,
    index: usize,
    start_var: usize,
    start_x: usize,
    start_y: usize,
) -> bool {
    if index == gift_numbers.len() {
        return true;
    }
    if gift_numbers[index] == 0 {
        return backtrack(grid, shapes, gift_numbers, index + 1, 0, 0, 0);
    }

    let rows = grid.len();
    let cols = grid[0].len();
    let max_x = rows.saturating_sub(2);
    let max_y = cols.saturating_sub(2);

    let vars = &shapes[index];
    for v_idx in start_var..vars.len() {
        let var = &vars[v_idx];
        let initial_x = if v_idx == start_var { start_x } else { 0 };
        for x in initial_x..max_x {
            let initial_y = if v_idx == start_var && x == start_x { start_y } else { 0 };
            for y in initial_y..max_y {
                if can_place(grid, var, x, y) {
                    for &(r, c) in &var.cells {
                        grid[x + r][y + c] = 1;
                    }

                    gift_numbers[index] -= 1;
                    let success = if gift_numbers[index] == 0 {
                        backtrack(grid, shapes, gift_numbers, index + 1, 0, 0, 0)
                    } else {
                        backtrack(grid, shapes, gift_numbers, index, v_idx, x, y)
                    };

                    for &(r, c) in &var.cells {
                        grid[x + r][y + c] = 0;
                    }
                    gift_numbers[index] += 1;

                    if success {
                        return true;
                    }
                }
            }
        }
    }
    false
}

pub fn part1() {
    let content = fs::read_to_string("day12_input.txt").expect("Should have been able to read the file");
    let lines: Vec<&str> = content.lines().collect();

    let mut bounding_dimensions: Vec<Vec<u64>> = Vec::new();
    let mut gift_numbers: Vec<Vec<u64>> = Vec::new();
    let mut raw_shapes: Vec<[[u64; 3]; 3]> = Vec::new();

    let mut i = 0;
    while i < lines.len() {
        let line = lines[i].trim();
        if line.is_empty() {
            i += 1;
            continue;
        }
        if line.contains(':') && line.contains('x') {
            let (dimension, gift_target) = line.split_once(": ").unwrap();
            bounding_dimensions.push(dimension.split('x').map(|x| x.parse::<u64>().unwrap()).collect());
            gift_numbers.push(gift_target.split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect());
            i += 1;
        } else if line.ends_with(':') {
            let mut shape = [[0u64; 3]; 3];
            for r in 0..3 {
                i += 1;
                let row_str = lines[i].trim();
                for (c, ch) in row_str.chars().enumerate() {
                    if ch == '#' {
                        shape[r][c] = 1;
                    }
                }
            }
            raw_shapes.push(shape);
            i += 1;
        } else {
            i += 1;
        }
    }

    let shapes_variations: Vec<Vec<Variation>> = raw_shapes.iter().map(|shape| {
        let mut set = HashSet::new();
        let mut curr = *shape;
        for _ in 0..4 {
            set.insert(curr);
            curr = rotate_90(&curr);
        }
        let mut flipped = flip_horizontal(&curr);
        for _ in 0..4 {
            set.insert(flipped);
            flipped = rotate_90(&flipped);
        }
        set.into_iter().map(|grid| {
            let mut cells = Vec::new();
            for r in 0..3 {
                for c in 0..3 {
                    if grid[r][c] == 1 {
                        cells.push((r, c));
                    }
                }
            }
            Variation { cells }
        }).collect()
    }).collect();

    let mut result = 0;
    for i in 0..bounding_dimensions.len() {
        let rows = bounding_dimensions[i][0] as usize;
        let cols = bounding_dimensions[i][1] as usize;

        let total_required_cells: u64 = gift_numbers[i].iter().enumerate().map(|(idx, &count)| {
            let cell_count = if !shapes_variations[idx].is_empty() {
                shapes_variations[idx][0].cells.len() as u64
            } else {
                0
            };
            count * cell_count
        }).sum();

        if total_required_cells > (rows * cols) as u64 {
            continue;
        }

        let mut grid = vec![vec![0u64; cols]; rows];
        let mut gifts = gift_numbers[i].clone();
        if backtrack(&mut grid, &shapes_variations, &mut gifts, 0, 0, 0, 0) {
            result += 1;
        }
    }
    println!("Result: {}", result);
}