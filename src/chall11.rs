use std::collections::{HashMap, HashSet};
use std::fs;

pub fn part1(){
    let content= fs::read_to_string("day11_input.txt").expect("Should have been able to read the file");
    let lines: Vec<&str> = content.lines().collect();
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in lines{
        let (node,edges) = line.split_once(": ").unwrap();
        let edges: Vec<&str> = edges.split(" ").filter(|s| !s.is_empty()).collect();
        graph.insert(node, edges);
    }
    println!("Graph: {:?}", graph);
    let mut number_of_paths = 0;
    //BFS to not miss any path from "you" to "out"
    let mut queue: Vec<(&str, Vec<&str>)> = vec![("you", vec!["you"])];
    while !queue.is_empty() {
        let (current_node, path) = queue.remove(0);
        if current_node == "out" {
            number_of_paths += 1;
            continue;
        }
        if let Some(neighbors) = graph.get(current_node) {
            for &neighbor in neighbors {
                if !path.contains(&neighbor) {
                    let mut new_path = path.clone();
                    new_path.push(neighbor);
                    queue.push((neighbor, new_path));
                }
            }
        }
    }
    println!("Number of paths: {}", number_of_paths);
}



fn number_of_paths<'a>(
    graph: &HashMap<&'a str, Vec<&'a str>>,
    start: &'a str,
    end: &'a str,
    memo: &mut HashMap<(&'a str, &'a str), usize>,
) -> usize {
    if start == end {
        return 1;
    }

    if let Some(&count) = memo.get(&(start, end)) {
        return count;
    }

    let mut total = 0;

    if let Some(neighbors) = graph.get(start) {
        for &next in neighbors {
            total += number_of_paths(graph, next, end, memo);
        }
    }

    memo.insert((start, end), total);
    total
}

pub fn part2() {
    let content = fs::read_to_string("day11_input.txt")
        .expect("Should have been able to read the file");

    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in content.lines() {
        let (node, edges) = line.split_once(": ").unwrap();
        graph.insert(node, edges.split_whitespace().collect());
    }

    let mut memo = HashMap::new();

    let svr_to_dac = number_of_paths(&graph, "svr", "dac", &mut memo);
    println!("Number of paths from svr to dac: {}", svr_to_dac);
    let svr_to_fft = number_of_paths(&graph, "svr", "fft", &mut memo);
    println!("Number of paths from svr to fft: {}", svr_to_fft);
    let dac_to_fft = number_of_paths(&graph, "dac", "fft", &mut memo);
    println!("Number of paths from dac to fft: {}", dac_to_fft);
    let fft_to_dac = number_of_paths(&graph, "fft", "dac", &mut memo);
    println!("Number of paths from fft to dac: {}", fft_to_dac);
    let fft_to_out = number_of_paths(&graph, "fft", "out", &mut memo);
    println!("Number of paths from fft to out: {}", fft_to_out);
    let dac_to_out = number_of_paths(&graph, "dac", "out", &mut memo);
    println!("Number of paths from dac to out: {}", dac_to_out);

    let number_of_paths =
        svr_to_dac * dac_to_fft * fft_to_out
        + svr_to_fft * fft_to_dac * dac_to_out;

    println!("Number of paths: {}", number_of_paths);
}