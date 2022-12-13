use core::panic;
use std::time::Instant;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn check_cycle_goal(cycle_goal:i32, current_cycle:i32, cpu_register:i32) -> Option<i32> {
    if cycle_goal == current_cycle {
        return Some(cycle_goal*cpu_register);
    }
    None
}

fn get_signal_sum(input: &str) -> i32 {
    let lines = read_lines(input)
        .unwrap_or_else(|_| panic!("File '{input}' should be accesible"));

    let mut parsed_line;
    let cycles = [20,60,100,140,180,220];
    let mut current_cycle_goal: usize = 0;
    let mut current_cycle = 0;
    let mut sum_signal_strength = 0;
    let mut cpu_register = 1;

    'outer: for line in lines {
        if let Ok(line) = line {
            parsed_line = line.split_whitespace();
            match parsed_line.next().unwrap() {
                "noop" => {
                    current_cycle += 1;
                    if let Some(signal_strength) = check_cycle_goal(
                        cycles[current_cycle_goal], current_cycle, cpu_register) {
                        sum_signal_strength += signal_strength;
                        current_cycle_goal += 1;
                        if current_cycle_goal == cycles.len() {break 'outer}
                    }
                }
                "addx" => {
                    for _ in 0..2 {
                        current_cycle += 1;
                        if let Some(signal_strength) = check_cycle_goal(
                            cycles[current_cycle_goal], current_cycle, cpu_register) {
                            sum_signal_strength += signal_strength;
                            current_cycle_goal += 1;
                            if current_cycle_goal == cycles.len() {break 'outer}
                        }
                    }
                    cpu_register += parsed_line.next().unwrap().parse::<i32>().unwrap();
                }
                _ => panic!("Can only be 'noop' or 'addx'"),
            }
            
        }
    }
    
    sum_signal_strength
}


fn check_cycle_goal_v2(cycle_goal:i32, current_cycle:i32, cpu_register:i32, pixels: &mut String) {
    let current_market_position = current_cycle - cycle_goal + 40;
    if cpu_register <= current_market_position && cpu_register+2 >= current_market_position {
        pixels.push('#');
    }
    else {
        pixels.push('.'); 
    }
}

fn draw_sprite(input: &str) -> Vec<String> {
    let lines = read_lines(input)
        .unwrap_or_else(|_| panic!("File '{input}' should be accesible"));

    let mut parsed_line;
    let mut pixel_matrix = vec![String::new(); 6];
    let cycles = [40,80,120,160,200,240];
    let mut current_cycle_goal: usize = 0;
    let mut current_cycle = 0;
    let mut cpu_register = 1;

    'outer: for line in lines {
        if let Ok(line) = line {
            parsed_line = line.split_whitespace();
            match parsed_line.next().unwrap() {
                "noop" => {
                    current_cycle += 1;
                    check_cycle_goal_v2(
                        cycles[current_cycle_goal], 
                        current_cycle, 
                        cpu_register,
                        &mut pixel_matrix[current_cycle_goal]);
                        if cycles[current_cycle_goal] == current_cycle {current_cycle_goal += 1;}
                        if current_cycle_goal == cycles.len() {break 'outer}
                }
                "addx" => {
                    for _ in 0..2 {
                        current_cycle += 1;
                        check_cycle_goal_v2(
                            cycles[current_cycle_goal], 
                            current_cycle, 
                            cpu_register,
                            &mut pixel_matrix[current_cycle_goal]);
                            if cycles[current_cycle_goal] == current_cycle {current_cycle_goal += 1;}
                            if current_cycle_goal == cycles.len() {break 'outer}
                    }
                    cpu_register += parsed_line.next().unwrap().parse::<i32>().unwrap();
                }
                _ => panic!("Can only be 'noop' or 'addx'"),
            }
            
        }
    }
    
    pixel_matrix
}

fn main() {
    println!("Starting execution!\n");
    let start = Instant::now();

    let input = "./input.txt";
    
    let signal_sum = get_signal_sum(input);
    let pixel_matrix = draw_sprite(input);

    println!("The sum of the singals is {signal_sum}\n");
    println!("Sprite:");
    for pixels in pixel_matrix {
        println!("{}", pixels);
    }
    

    let duration = start.elapsed();
    println!("\nTime elapsed is: {:?}", duration);
}
