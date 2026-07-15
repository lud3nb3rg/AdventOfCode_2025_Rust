use std::fs;

pub fn part1(){
    let contents=fs::read_to_string("day6_input.txt").expect("Should have been able to read the file");
    let lines: Vec<&str>=contents.lines().collect();
    let mut sum_results=0;
    let first_numbers=lines[0].trim().split_whitespace().map(|num_str| num_str.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    let second_numbers=lines[1].trim().split_whitespace().map(|num_str| num_str.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    let third_numbers=lines[2].trim().split_whitespace().map(|num_str| num_str.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    let fourth_numbers=lines[3].trim().split_whitespace().map(|num_str| num_str.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    let operations=lines[4].trim().split_whitespace().collect::<Vec<&str>>();
    for i in 0..operations.len() {
        let op=operations[i];
        let first=first_numbers[i];
        let second=second_numbers[i];
        let third=third_numbers[i];
        let fourth=fourth_numbers[i];
        let result=match op {
            "+" => first + second + third + fourth,
            "-" => first - second - third - fourth,
            "*" => first * second * third * fourth,
            "/" => first / second / third / fourth,
            _ => panic!("Unknown operation: {}", op),
        };
        sum_results += result;
    }
    println!("Sum of results: {}", sum_results);
}

fn default_op(op: &str) -> u64 {
    match op {
        "+" => 0,
        "-" => 0,
        "*" => 1,
        "/" => 1,
        _ => panic!("Unknown operation: {}", op),
    }
}
pub fn part2(){
    let contents=fs::read_to_string("day6_input.txt").expect("Should have been able to read the file");
    let lines: Vec<&str>=contents.lines().collect();
    let mut sum_results=0;
    let mut i =0;
    while i<lines[0].len(){
        if lines[4].chars().nth(i)!=Some(' '){
            let op=&lines[4][i..i+1];
            let mut numbers=Vec::new();
            numbers.push(
                (lines[0][i..i+1].to_string()+&lines[1][i..i+1]+&lines[2][i..i+1]+&lines[3][i..i+1]).trim().parse::<u64>().unwrap_or(default_op(op))
            );
            i+=1;
            while lines[4].chars().nth(i)==Some(' '){
                println!("{}",lines[0][i..i+1].to_string()+&lines[1][i..i+1]+&lines[2][i..i+1]+&lines[3][i..i+1]);
                numbers.push(
                    (lines[0][i..i+1].to_string()+&lines[1][i..i+1]+&lines[2][i..i+1]+&lines[3][i..i+1]).trim().parse::<u64>().unwrap_or(default_op(op))
                );
                i+=1;
            }
            let result=match op {
                "+" => numbers.iter().sum(),
                "-" => numbers[0] - numbers[1..].iter().sum::<u64>(),
                "*" => numbers.iter().product(),
                "/" => numbers[0] / numbers[1..].iter().product::<u64>(),
                _ => panic!("Unknown operation: {}", op),
            };
            println!("Operation: {}, Numbers: {:?}, Result: {}", op, numbers, result);
            sum_results += result;
        }
    }
    println!("Sum of results: {}", sum_results);
}