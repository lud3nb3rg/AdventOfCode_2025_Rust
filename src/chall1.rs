use std::fs;

pub fn part1(){
    let contents=fs::read_to_string("day1_input.txt").expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.lines().collect();
    let mut res=0;
    let mut dial=50;
    for line in lines{
        if line.chars().nth(0).unwrap()=='L'{
            dial=(dial-line[1..].parse::<i32>().unwrap()) % 100;
        } else {
            dial=(dial+line[1..].parse::<i32>().unwrap()) % 100;
        }
        if dial==0{
            res+=1;
        }
    }
    println!("The dial was at 0 {} times", res);
}

pub fn part2(){
    let contents=fs::read_to_string("day1_input.txt").expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.lines().collect();
    let mut res=0;
    let mut dial=50;
    for line in lines{
        println!("Dial: {}, Res: {}, Line: {}", dial, res, line);
        if line.chars().nth(0).unwrap()=='L'{
            let mut turn = line[1..].parse::<i32>().unwrap();
            res+=turn/100;
            turn=turn%100;
            if dial-turn<=0 && dial!=0{
                res+=1;
            }
            dial=(dial-turn) % 100;
            if dial<=0{
                dial=(dial+100)%100;
            }
        } else {
            let mut turn = line[1..].parse::<i32>().unwrap();
            res+=turn/100;
            turn=turn%100;
            if dial+turn>=100{
                res+=1;
            }
            dial=(dial+turn) % 100;
        }
    }
    println!("The dial was at 0 {} times", res);
}
