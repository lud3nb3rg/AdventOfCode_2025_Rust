use std::fs;

pub fn part1(){
    let contents=fs::read_to_string("day5_input.txt").expect("Should have been able to read the file");
    let lines: Vec<&str>=contents.lines().collect();
    let empty_line_index=lines.iter().position(|&line| line.trim().is_empty()).unwrap();
    let ranges: Vec<(u64, u64)>=lines[..empty_line_index].iter().map(|line: &&str| {
        let bounds: Vec<&str> = line.trim().split("-").collect();
        return (bounds[0].parse::<u64>().unwrap(), bounds[1].parse::<u64>().unwrap());
    }).collect();
    let ingredients: Vec<u64>=lines[empty_line_index+1..].iter().map(|line: &&str| {
        return line.trim().parse::<u64>().unwrap();
    }).collect();
    let mut fresh_count=0;
    for ingredient in ingredients {
        let mut is_fresh=false;
        for (lower, upper) in &ranges {
            if ingredient >= *lower && ingredient <= *upper {
                is_fresh=true;
                break;
            }
        }
        if is_fresh {
            fresh_count += 1;
        }
    }
    println!("Number of fresh ingredients: {}", fresh_count);
}

pub fn part2(){
    let contents=fs::read_to_string("day5_input.txt").expect("Should have been able to read the file");
    let lines: Vec<&str>=contents.lines().collect();
    let empty_line_index=lines.iter().position(|&line| line.trim().is_empty()).unwrap();
    let mut ranges: Vec<(u64, u64)>=lines[..empty_line_index].iter().map(|line: &&str| {
        let bounds: Vec<&str> = line.trim().split("-").collect();
        return (bounds[0].parse::<u64>().unwrap(), bounds[1].parse::<u64>().unwrap());
    }).collect();
    ranges.sort_by(|a, b| a.0.cmp(&b.0));
    let mut merged_ranges: Vec<(u64, u64)> = Vec::new();
    merged_ranges.push(ranges[0]);
    for (lower, upper) in ranges.iter().skip(1) {
        let last_range = merged_ranges.last_mut().unwrap();
        if *lower <= last_range.1 + 1 {
            last_range.1 = std::cmp::max(last_range.1, *upper);
        } else {
            merged_ranges.push((*lower, *upper));
        }
    }
    let fresh_ingredients_number: u64 = merged_ranges.iter().map(|(lower, upper)| upper - lower + 1).sum();
    println!("Number of fresh ingredients: {}", fresh_ingredients_number);
}