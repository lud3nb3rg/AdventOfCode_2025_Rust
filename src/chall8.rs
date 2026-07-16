use std::fs;
use std::collections::BinaryHeap;
use std::cmp::Reverse;
struct UFNode {
    parent: usize,
    rank: usize,
}
fn union_find_make_set(n: usize) -> Vec<UFNode> {
    let mut uf = Vec::with_capacity(n);
    for i in 0..n {
        uf.push(UFNode { parent: i, rank: 0 });
    }
    uf
}
fn union_find_find(uf: &mut Vec<UFNode>, x: usize) -> usize {
    if uf[x].parent != x {
        let p = uf[x].parent;          // read before the reborrow
        uf[x].parent = union_find_find(uf, p);
    }
    uf[x].parent
}
fn union_find_union(uf: &mut Vec<UFNode>, x: usize, y: usize) {
    let x_root = union_find_find(uf, x);
    let y_root = union_find_find(uf, y);
    if x_root == y_root {
        return;
    }
    if uf[x_root].rank < uf[y_root].rank {
        uf[x_root].parent = y_root;
    } else if uf[x_root].rank > uf[y_root].rank {
        uf[y_root].parent = x_root;
    } else {
        uf[y_root].parent = x_root;
        uf[x_root].rank += 1;
    }
}
pub fn part1(){
    let content=fs::read_to_string("day8_input.txt").expect("Error reading file");
    let lines: Vec<&str> = content.lines().collect();
    let coords: Vec<(u64, u64, u64)> = lines.iter().map(|line| {
        let parts: Vec<&str> = line.split(',').collect();
        (parts[0].parse::<u64>().unwrap(), parts[1].parse::<u64>().unwrap(), parts[2].parse::<u64>().unwrap())
    }).collect();
    //order coord pairs by smallest euclidean distance between the two coords
    let mut heap = BinaryHeap::new();
    for i in 0..coords.len() {
        for j in i+1..coords.len() {
            let dist = ((coords[i].0 as i128- coords[j].0 as i128).pow(2) + (coords[i].1 as i128 - coords[j].1 as i128).pow(2) + (coords[i].2 as i128 - coords[j].2 as i128).pow(2)) as u64;
            heap.push((Reverse(dist), i, j));
        }
    }
    // Connect the closest pairs for the 1000 closest pairs
    let mut uf = union_find_make_set(coords.len());
    for _ in 0..1000 {
        if let Some((Reverse(_), i, j)) = heap.pop() {
            union_find_union(&mut uf, i, j);
        }
    }
    // Multiply the sizes of the three largest sets
    let mut set_sizes = vec![0; coords.len()];
    for i in 0..coords.len() {
        set_sizes[union_find_find(&mut uf, i)] += 1;
    }
    set_sizes.sort_by(|a, b| b.cmp(a));
    println!("Set sizes: {:?}", set_sizes);
    println!("Result: {}", set_sizes[0] * set_sizes[1] * set_sizes[2]);
}

pub fn part2(){
    //Same thing except we link the closest until everything is connected, then we print the product of the X coordinates of the last connected pair
    let content=fs::read_to_string("day8_input.txt").expect("Error reading file");
    let lines: Vec<&str> = content.lines().collect();
    let coords: Vec<(u64, u64, u64)> = lines.iter().map(|line| {
        let parts: Vec<&str> = line.split(',').collect();
        (parts[0].parse::<u64>().unwrap(), parts[1].parse::<u64>().unwrap(), parts[2].parse::<u64>().unwrap())
    }).collect();
    //order coord pairs by smallest euclidean distance between the two coords
    let mut heap = BinaryHeap::new();
    for i in 0..coords.len() {
        for j in i+1..coords.len() {
            let dist = ((coords[i].0 as i128- coords[j].0 as i128).pow(2) + (coords[i].1 as i128 - coords[j].1 as i128).pow(2) + (coords[i].2 as i128 - coords[j].2 as i128).pow(2)) as u64;
            heap.push((Reverse(dist), i, j));
        }
    }
    // Connect the closest pairs until everything is connected
    let mut uf = union_find_make_set(coords.len());
    let mut last_connected_pair = None;
    while let Some((Reverse(_), i, j)) = heap.pop() {
        if union_find_find(&mut uf, i) != union_find_find(&mut uf, j) {
            union_find_union(&mut uf, i, j);
            last_connected_pair = Some((i, j));
        }
    }
    // Print the product of the X coordinates of the last connected pair
    if let Some((i, j)) = last_connected_pair {
        println!("Last connected pair: ({}, {})", coords[i].0, coords[j].0);
        println!("Product of X coordinates: {}", coords[i].0 * coords[j].0);
    }
}