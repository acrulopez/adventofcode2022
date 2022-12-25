use std::time::Instant;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;
use std::cmp;

const MINUTES_LEFT_V1:i32 = 24;
const MINUTES_LEFT_V2:i32 = 32;

#[derive(Debug)]
struct RobotCosts {
    ore: i32,
    clay: i32,
    obsidian: (i32, i32),
    geode: (i32, i32)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_input(input: &str) -> Vec<RobotCosts> {
    let lines = read_lines(input)
        .unwrap_or_else(|_| panic!("File '{input}' should be accesible"));


    let mut robots_cost = vec![];
    let re = Regex::new(r"\d+").unwrap();

    for line in lines {
        if let Ok(line) = line {
            let mut robot_costs_it = line.split('.');

            // Ore
            let ore = re
            .find_iter(&robot_costs_it.next().unwrap())
            .skip(1).next().unwrap()
            .as_str().parse().unwrap();

            // Clay
            let clay = re
            .find_iter(&robot_costs_it.next().unwrap())
            .next().unwrap()
            .as_str().parse().unwrap();

            // Obsidian
            let mut obsidian = re
            .find_iter(&robot_costs_it.next().unwrap())
            .map(|x| x.as_str().parse::<i32>().unwrap());
            let obsidian = (obsidian.next().unwrap(), obsidian.next().unwrap());

            // Geode
            let mut geode = re
            .find_iter(&robot_costs_it.next().unwrap())
            .map(|x| x.as_str().parse::<i32>().unwrap());
            let geode = (geode.next().unwrap(), geode.next().unwrap());


            robots_cost.push(RobotCosts {
                ore,
                clay,
                obsidian,
                geode
            })
        }
    }

    robots_cost
}

pub fn div_up(a: i32, b: i32) -> i32 {
    (a + (b - 1))/b
}

fn _max_geode(robot_cost: &RobotCosts, mut minutes_left:i32, mut materials: [i32; 4], mut robots: [i32; 4], next_robot:usize, prunning_treshold: i32) -> i32 {

    let minutes_needed;
    match next_robot {
        0 => {

            let ore_needed = cmp::max(robot_cost.ore - materials[0], 0);
            minutes_needed = div_up(ore_needed, robots[0]) + 1;

            // No time enough for the robot to produce anything
            if minutes_needed > minutes_left {
                return materials[3] + minutes_left*robots[3];
            }

            materials[0] -= robot_cost.ore;

        },
        1 => {

            let ore_needed = cmp::max(robot_cost.clay - materials[0],0);
            minutes_needed = div_up(ore_needed, robots[0]) + 1;

            // No time enough for the robot to produce anything
            if minutes_needed > minutes_left {
                return materials[3] + minutes_left*robots[3];
            }

            materials[0] -= robot_cost.clay;
        }
        2 => {

            // Compute time needed to create that robot
            let ore_needed = cmp::max(robot_cost.obsidian.0 - materials[0],0);
            let clay_needed = cmp::max(robot_cost.obsidian.1 - materials[1],0);

            let time_needed_1 = div_up(ore_needed, robots[0]) + 1;
            let time_needed_2 = div_up(clay_needed, robots[1]) + 1;

            minutes_needed = cmp::max(time_needed_1, time_needed_2);

            // No time enough for the robot to produce anything
            if minutes_needed > minutes_left {
                return materials[3] + minutes_left*robots[3];
            }

            materials[0] -= robot_cost.obsidian.0;
            materials[1] -= robot_cost.obsidian.1;
        }
        3 => {

            // Compute time needed to create that robo
            let ore_needed = cmp::max(robot_cost.geode.0 - materials[0],0);
            let obsidian_needed = cmp::max(robot_cost.geode.1 - materials[2],0);

            let time_needed_1 = div_up(ore_needed, robots[0]) + 1;
            let time_needed_2 = div_up(obsidian_needed, robots[2]) + 1;

            minutes_needed = cmp::max(time_needed_1, time_needed_2);

            // No time enough for the robot to produce anything
            if minutes_needed > minutes_left {
                return materials[3] + minutes_left*robots[3];
            }

            materials[0] -= robot_cost.geode.0;
            materials[2] -= robot_cost.geode.1;
        }
        _ => panic!("Next robot can only be [0,1,2,3]")
    }

    // Take the time
    minutes_left -= minutes_needed;

    // Increase the materials
    for i in 0..4 {
        materials[i] += robots[i] * minutes_needed;
    }

    // Create the robot
    robots[next_robot] += 1;

    
    if minutes_left < prunning_treshold && robots[3] == 0 {
        return 0;
    }

    let max_geodes;
    if next_robot == 0 {
        let geodes_0 = _max_geode(robot_cost, minutes_left, materials, robots, 0, prunning_treshold);
        let geodes_1 = _max_geode(robot_cost, minutes_left, materials, robots, 1, prunning_treshold);
        max_geodes = cmp::max(geodes_0, geodes_1);
    }
    else if next_robot == 1 {
        let geodes_0 = _max_geode(robot_cost, minutes_left, materials, robots, 0, prunning_treshold);
        let geodes_1 = _max_geode(robot_cost, minutes_left, materials, robots, 1, prunning_treshold);
        let geodes_2 = _max_geode(robot_cost, minutes_left, materials, robots, 2, prunning_treshold);
        max_geodes = cmp::max(geodes_0, cmp::max(geodes_1, geodes_2));
    }
    else if next_robot == 2 {
        let geodes_1 = _max_geode(robot_cost, minutes_left, materials, robots, 1,prunning_treshold);
        let geodes_2 = _max_geode(robot_cost, minutes_left, materials, robots, 2, prunning_treshold);
        let geodes_3 = _max_geode(robot_cost, minutes_left, materials, robots, 3, prunning_treshold);
        max_geodes = cmp::max(geodes_1, cmp::max(geodes_2, geodes_3));
    }
    else {
        let geodes_2 = _max_geode(robot_cost, minutes_left, materials, robots, 2, prunning_treshold);
        let geodes_3 = _max_geode(robot_cost, minutes_left, materials, robots, 3, prunning_treshold);
        max_geodes = cmp::max(geodes_2, geodes_3);
    }

    max_geodes

}

fn max_geodes(robot_cost: &RobotCosts, minutes_left:i32, prunning_treshold:i32) -> i32 {

    let materials = [0,0,0,0];
    let robots = [1,0,0,0];


    // Compute all possible paths and return the best one
    let max_geodes_0 = _max_geode(robot_cost, minutes_left, materials, robots, 0, prunning_treshold);

    let max_geodes_1 = _max_geode(robot_cost, minutes_left, materials, robots, 1, prunning_treshold);

    cmp::max(max_geodes_0, max_geodes_1)
}

fn solution_v1(input: &str) -> i32 {
    let robots_costs = parse_input(input);

    let mut sum = 0;
    for (it, robot_cost) in robots_costs.iter().enumerate() {
        sum += max_geodes(robot_cost, MINUTES_LEFT_V1, 5) * (it+1) as i32;
    }

    sum
}

fn solution_v2(input: &str) -> i32 {
    let robots_costs = parse_input(input);
    let robots_costs = &robots_costs[0..3];

    let mut prod = 1;
    for robot_cost in robots_costs.iter() {
        prod *= max_geodes(robot_cost, MINUTES_LEFT_V2, 7);
    }

    prod

}

fn main() {
    let input = "./input.txt";

    // --------------------------V1--------------------------------

    println!("Starting execution version 1!\n");
    let start = Instant::now();
    
    let solution_v1 = solution_v1(input);
    println!("V1 solution is {}", solution_v1);

    let duration = start.elapsed();
    println!("\nTime elapsed is: {:?}", duration);
    
    // --------------------------V2--------------------------------

    println!("Starting execution version 2!\n");
    let start = Instant::now();

    let solution_v2 = solution_v2(input);
    println!("V2 solution is {}", solution_v2);
    

    let duration = start.elapsed();
    println!("\nTime elapsed is: {:?}", duration);
}
