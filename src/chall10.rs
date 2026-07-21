use std::fs;
use z3::{Config, Context, Optimize, SatResult, ast::Int};
fn state_to_index_part1(state: &Vec<bool>) -> usize {
    state
        .iter()
        .enumerate()
        .fold(0, |acc, (i, &b)| acc + ((b as usize) << i))
}
fn solve_quickest_toggle_path_part1(
    target: &Vec<bool>,
    toggles: &Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
) -> usize {
    //BFS to find the shortest path to the target state
    let mut queue = std::collections::VecDeque::new();
    let mut initial_state = vec![false; target.len()];
    queue.push_back((initial_state.clone(), 0));
    while queue.len() > 0 {
        let (current_state, steps) = queue.pop_front().unwrap();
        if current_state == *target {
            return steps;
        }
        let index = state_to_index_part1(&current_state);
        if visited[index] {
            continue;
        }
        visited[index] = true;
        for toggle in toggles {
            let mut next_state = current_state.clone();
            for &i in toggle {
                next_state[i] = !next_state[i];
            }
            queue.push_back((next_state, steps + 1));
        }
    }
    panic!("No path found");
}
pub fn part1() {
    let content =
        fs::read_to_string("day10_input.txt").expect("Should have been able to read the file");
    let lines: Vec<&str> = content.lines().collect();
    let mut steps_sum = 0;
    for line in lines {
        let (str_target, str_toggles) = line
            .trim()
            .split_once(" ")
            .expect("Line should contain a target and toggles");
        let tmp_toggles: Vec<Vec<usize>> = str_toggles
            .split(" ")
            .map(|s| {
                s[1..s.len() - 1]
                    .split(",")
                    .filter_map(|v| v.parse::<usize>().ok())
                    .collect()
            })
            .collect();
        let toggles = tmp_toggles[..tmp_toggles.len() - 1].to_vec();
        let target: Vec<bool> = str_target
            .split("")
            .filter(|v| *v != "[" && *v != "]" && *v != "")
            .map(|v| v == "#")
            .collect();
        println!("Target: {:?}, Toggles: {:?}", target, toggles);
        let mut visited = vec![false; 2_usize.pow(target.len() as u32)];
        let steps = solve_quickest_toggle_path_part1(&target, &toggles, &mut visited);
        steps_sum += steps
    }
    println!("Total steps: {}", steps_sum);
}

pub fn part2() {
    

    let content = fs::read_to_string("day10_input.txt").expect("Could not read input");

    let mut total_presses = 0i64;

    for line in content.lines() {
        let parts: Vec<&str> = line.trim().split(" ").collect();

        let button_strs = &parts[1..parts.len() - 1];
        let target_str = parts[parts.len() - 1];

        let buttons: Vec<Vec<usize>> = button_strs
            .iter()
            .map(|s| {
                s[1..s.len() - 1]
                    .split(",")
                    .filter_map(|x| x.parse::<usize>().ok())
                    .collect()
            })
            .collect();

        let target: Vec<i64> = target_str[1..target_str.len() - 1]
            .split(",")
            .filter_map(|x| x.parse::<i64>().ok())
            .collect();

        let cfg = Config::new();
        let ctx = Context::thread_local();

        let optimizer = Optimize::new();
        let presses: Vec<Int> = (0..buttons.len())
            .map(|i| Int::new_const(format!("x{}", i)))
            .collect();
        for x in &presses {
            optimizer.assert(&x.ge(&Int::from_i64(0)));
        }
        for counter in 0..target.len() {
            let mut sum = Int::from_i64(0);

            for (button_idx, button) in buttons.iter().enumerate() {
                if button.contains(&counter) {
                    sum = sum + &presses[button_idx];
                }
            }

            optimizer.assert(&sum.eq(&Int::from_i64(target[counter])));
        }
        let total = presses.iter().fold(Int::from_i64(0), |acc, x| &acc + x);

        optimizer.minimize(&total);

        let objective = optimizer.minimize(&total);

        match optimizer.check(&[]) {
            SatResult::Sat => {}
            _ => panic!("No solution found"),
            SatResult::Unsat => todo!(),
            SatResult::Unknown => todo!(),
        }

        let model = optimizer.get_model().expect("No model returned");
        let solution: Vec<i64> = presses
            .iter()
            .map(|x| model.eval(x, true).unwrap().as_i64().unwrap())
            .collect();

        let answer = model.eval(&total, true).unwrap().as_i64().unwrap();

        println!("Buttons pressed: {:?}", solution);
        println!("Minimum presses: {}", answer);

        total_presses += answer;
    }

    println!("Total presses: {}", total_presses);
}
