use std::fs;

pub fn part1(){
    let content=fs::read_to_string("day2_input.txt").expect("Should have been able to read the file");
    let ranges: Vec<&str> = content.trim().split(',').collect();
    let mut sum=0;
    for range in ranges{
        let bounds: Vec<&str> = range.split('-').collect();
        let lower_bound=bounds[0].parse::<i64>().unwrap();
        let upper_bound=bounds[1].parse::<i64>().unwrap();
        for i in lower_bound..=upper_bound{
            let string_num=i.to_string();
            if (string_num[..string_num.len()/2]==string_num[string_num.len()/2..]){
                sum+=i;
            }
        }
    }
    println!("Sum: {}", sum);
}

pub fn part2(){
    let content=fs::read_to_string("day2_input.txt").expect("Should have been able to read the file");
    let ranges: Vec<&str> = content.trim().split(',').collect();
    let mut sum=0;
    for range in ranges{
        let bounds: Vec<&str> = range.split('-').collect();
        let lower_bound=bounds[0].parse::<i64>().unwrap();
        let upper_bound=bounds[1].parse::<i64>().unwrap();
        for i in lower_bound..=upper_bound{
            let string_num=i.to_string();
            for j in 1..=string_num.len()/2 {
                if string_num==string_num[..j].repeat(string_num.len()/j){
                    sum+=i;
                    break;
                }
            }
        }
    }
    println!("Sum: {}", sum);
}