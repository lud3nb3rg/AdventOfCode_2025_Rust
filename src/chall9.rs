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

pub fn part2(){
    let content=fs::read_to_string("day9_input.txt").expect("Error reading file");
    let lines: Vec<&str>= content.lines().collect();
    let coords: Vec<(u64, u64)> = lines.iter().map(|line| {
        let parts: Vec<&str> = line.split(',').collect();
        (parts[0].parse::<u64>().unwrap(), parts[1].parse::<u64>().unwrap())
    }).collect();
}