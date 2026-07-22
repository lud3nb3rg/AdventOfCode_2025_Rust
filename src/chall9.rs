use std::fs;

pub fn part1(){
    let content=fs::read_to_string("day9_input.txt").expect("Error reading file");
    let lines: Vec<&str> = content.lines().collect();
    let coords: Vec<(u64, u64)> = lines.iter().map(|line| {
        let parts: Vec<&str> = line.split(',').collect();
        (parts[0].parse::<u64>().unwrap(), parts[1].parse::<u64>().unwrap())
    }).collect();
    let coord_pairs: Vec<((u64, u64), (u64, u64))> = coords.iter().enumerate().flat_map(|(i, &coord1)| {
        coords.iter().skip(i + 1).map(move |&coord2| (coord1, coord2))
    }).collect();
    let max_rectangle_area=coord_pairs.iter().map(|(a, b)| ((a.0 as i64-b.0 as i64).abs()+1)*((a.1 as i64-b.1 as i64).abs()+1)).max().unwrap();
    println!("Max rectangle area: {}", max_rectangle_area);
}

#[derive(Debug, Clone, Copy)]
enum Edge {
    Horizontal { y: u64, x_min: u64, x_max: u64 },
    Vertical { x: u64, y_min: u64, y_max: u64 },
}

pub fn part2() {
    let content = fs::read_to_string("day9_input.txt").expect("Error reading file");
    let lines: Vec<&str> = content.lines().collect();
    let coords: Vec<(u64, u64)> = lines
        .iter()
        .map(|line| {
            let parts: Vec<&str> = line.split(',').collect();
            (parts[0].parse::<u64>().unwrap(), parts[1].parse::<u64>().unwrap())
        })
        .collect();

    let n = coords.len();
    let mut edges = Vec::with_capacity(n);
    for i in 0..n {
        let (x1, y1) = coords[i];
        let (x2, y2) = coords[(i + 1) % n];
        if x1 == x2 {
            edges.push(Edge::Vertical {
                x: x1,
                y_min: y1.min(y2),
                y_max: y1.max(y2),
            });
        } else if y1 == y2 {
            edges.push(Edge::Horizontal {
                y: y1,
                x_min: x1.min(x2),
                x_max: x1.max(x2),
            });
        } else {
            panic!("Diagonal edge detected between {:?} and {:?}", coords[i], coords[(i+1)%n]);
        }
    }

    let mut max_area = 0u64;

    for i in 0..n {
        for j in (i + 1)..n {
            let (x1, y1) = coords[i];
            let (x2, y2) = coords[j];

            let rx1 = x1.min(x2);
            let rx2 = x1.max(x2);
            let ry1 = y1.min(y2);
            let ry2 = y1.max(y2);

            let area = (rx2 - rx1 + 1) * (ry2 - ry1 + 1);

            if area <= max_area {
                continue;
            }

            let mut edge_intersects = false;
            for edge in &edges {
                match edge {
                    Edge::Vertical { x, y_min, y_max } => {
                        if rx1 < *x && *x < rx2 && *y_max > ry1 && *y_min < ry2 {
                            edge_intersects = true;
                            break;
                        }
                    }
                    Edge::Horizontal { y, x_min, x_max } => {
                        if ry1 < *y && *y < ry2 && *x_max > rx1 && *x_min < rx2 {
                            edge_intersects = true;
                            break;
                        }
                    }
                }
            }

            if edge_intersects {
                continue;
            }
            let mx = (rx1 + rx2) as f64 / 2.0;
            let my = (ry1 + ry2) as f64 / 2.0;
            let test_y = my + 0.0001;
            let mut intersections = 0;
            for edge in &edges {
                if let Edge::Vertical { x, y_min, y_max } = edge {
                    if (*x as f64) > mx && (*y_min as f64) < test_y && test_y < (*y_max as f64) {
                        intersections += 1;
                    }
                }
            }

            if intersections % 2 == 1 {
                max_area = area;
            }
        }
    }

    println!("Max rectangle area inside polygon: {}", max_area);
}
