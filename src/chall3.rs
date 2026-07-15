use std::fs;

pub fn part1(){
    let contents=fs::read_to_string("day3_input.txt").expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.lines().collect();
    let mut sum=0;
    for line in lines{
        let mut ints=line.trim().split("").map(|x| x.parse::<i32>().unwrap_or(0)).collect::<Vec<i32>>();
        ints=ints[1..ints.len()-1].to_vec();
        let idxmax=ints.iter().position(|x| x==ints[..ints.len()-1].iter().max().unwrap()).unwrap();
        sum+=ints[idxmax]*10+ints[idxmax+1..].iter().max().unwrap();
    }
    println!("Sum: {}", sum);
}

pub fn part2(){
    let contents=fs::read_to_string("day3_input.txt").expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.lines().collect();
    let mut sum: u64=0;
    for line in lines{
        let mut ints=line.trim().split("").map(|x| x.parse::<i32>().unwrap_or(0)).collect::<Vec<i32>>();
        ints=ints[1..ints.len()-1].to_vec();
        let mut idxmax: isize=-1;
        for i in 0..12{
            let adapted_ints=ints.as_slice()[(idxmax+1) as usize..ints.len()-(11-i)].to_vec();
            idxmax=adapted_ints.iter().position(|x| x==adapted_ints.iter().max().unwrap()).unwrap() as isize+(idxmax+1);
            sum+=ints[idxmax as usize] as u64*(10_u64.pow((11-i) as u32));
        }
    }
    println!("Sum: {}", sum);
}