use std::time::Instant;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;
use std::collections::{HashMap, VecDeque};
// use rayon::prelude::*;
use itertools::Itertools;

const MINUTES_LEFT:u32 = 30;

#[derive(Debug)]
struct Valve {
    flow_rate: u32,
    leads_to: Vec<usize>
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_input(input: &str) -> (Vec<Valve>, usize){
    let lines = read_lines(input)
        .unwrap_or_else(|_| panic!("File '{input}' should be accesible"));


    let mut valve_name_to_number = HashMap::new();
    let mut valves: Vec<Valve> = vec![];
    let mut valve_names_exits: Vec<Vec<String>> = vec![];
    let mut start_at = 0;

    let re = Regex::new(r"([A-Z][A-Z])|\d+").unwrap();
    for (idx, line) in lines.enumerate() {
        if let Ok(line) = line {
            let mut line_splitted = re.find_iter(&line)
            .filter_map(|digits| Some(digits.as_str()));

            // Gets the name of the valve and assigns an idx to it
            let valve_name = line_splitted.next().unwrap().to_string();
            if valve_name == "AA" {
                start_at = idx;
            }
            valve_name_to_number.insert(valve_name, idx);
            

            // Creates the valve with its flow rate
            valves.push(Valve {
                flow_rate: line_splitted.next().unwrap().parse().unwrap(), 
                leads_to: vec![]
            });

            // Save the names of the exit valves
            valve_names_exits.push(vec![]);
            for valve_name in line_splitted {
                valve_names_exits.last_mut().unwrap().push(valve_name.to_string());
            }
        }
    }

    for (valve, valves_name) in valves.iter_mut().zip(valve_names_exits) {
        for valve_name in valves_name {
            valve.leads_to.push(valve_name_to_number[&valve_name]);
        }
    }

    (valves, start_at)

}



fn get_distance_matrix(valves: &Vec<Valve>, non_zero_valves: &Vec<usize>, start_at: usize) 
-> Vec<Vec<u32>> {
    let n = valves.len();
    let mut distances = vec![vec![50; n]; n];
    let mut queue = VecDeque::new();
    let mut non_zero_valves = non_zero_valves.clone();
    non_zero_valves.push(start_at);

    // For each valve in the network...
    for i in non_zero_valves.into_iter() {
        
        // Add the starting valve to the queue with a distance of 0.
        queue.push_back((i, 0));
        distances[i][i] = 0;

        'queue: while let Some((valve_idx, distance)) = queue.pop_front() { 
            if distance > MINUTES_LEFT-1 {break 'queue} // Means that is unreacheable in 30 times
            for goes_to in &valves[valve_idx].leads_to {
                let new_distance = distance + 1;
                if new_distance < distances[i][*goes_to] {
                    distances[i][*goes_to] = new_distance;
                    queue.push_back((*goes_to, new_distance));
                }
            }
        }
    }

    distances

}

fn compute_pressure(
    valves: &Vec<Valve>, 
    distance_matrix: &Vec<Vec<u32>>, 
    permutation: Vec<usize>,
    start_at: usize,
    mut minutes_left: u32) -> u32 {

    let mut pressure = 0;
    let mut last_valve_idx = start_at;

    for valve_idx in permutation {
        if distance_matrix[last_valve_idx][valve_idx] >= minutes_left - 1 {break;}
        minutes_left -= distance_matrix[last_valve_idx][valve_idx] + 1;
        pressure += valves[valve_idx].flow_rate * minutes_left;
        last_valve_idx = valve_idx;
    }

    pressure
}

fn compute_maximum_pressure_v1(
    valves: &Vec<Valve>, 
    distance_matrix: &Vec<Vec<u32>>, 
    non_zero_valves: &Vec<usize>,
    start_at: usize)
    -> u32 {

    let permutation_len = non_zero_valves.len()/2 + 1;


    // Parallel version
    // let maximum_pressure = non_zero_valves.clone().into_iter()
    //     .permutations(permutation_len)
    //     .par_bridge()
    //     .map(|x| compute_pressure(valves, distance_matrix, x, start_at, MINUTES_LEFT))
    //     .reduce(|| 0, |x, y| if x > y {return x} else {return y});

    // Serial version
    let maximum_pressure = non_zero_valves.clone().into_iter()
        .permutations(permutation_len)
        .map(|x| {compute_pressure(valves, distance_matrix, x, start_at, MINUTES_LEFT)})
        .reduce(|x, y| if x > y {return x} else {return y}).unwrap();


    maximum_pressure
}

fn compute_maximum_pressure_v2(
    valves: &Vec<Valve>, 
    distance_matrix: &Vec<Vec<u32>>, 
    non_zero_valves: &Vec<usize>,
    start_at: usize)
    -> u32 {

    let combinations = get_combinations(non_zero_valves, non_zero_valves.len()/2);
    
    let mut max = 0;
    for (human_comb, eleph_comb) in combinations.iter() {

        let maximum_pressure_human = human_comb.clone().into_iter()
            .permutations(human_comb.len())
            .map(|x| {compute_pressure(valves, distance_matrix, x, start_at, MINUTES_LEFT-4)})
            .reduce(|x, y| if x > y {return x} else {return y}).unwrap();

            
        let maximum_pressure_elph = eleph_comb.clone().into_iter()
            .permutations(eleph_comb.len())
            .map(|x| {compute_pressure(valves, distance_matrix, x, start_at, MINUTES_LEFT-4)})
            .reduce(|x, y| if x > y {return x} else {return y}).unwrap();

        if maximum_pressure_elph + maximum_pressure_human > max {
            max = maximum_pressure_elph + maximum_pressure_human;
        }
    }

    max
}

fn solution(input: &str) -> (u32, u32) {
    
    let (valves, start_at) =  parse_input(input);

    let non_zero_valves: Vec<usize> = valves.iter().enumerate()
        .filter_map(|(it, x)| if x.flow_rate != 0 {return Some(it);} else {return None;})
        .collect::<Vec<usize>>();

    let distance_matrix = get_distance_matrix(&valves, &non_zero_valves, start_at);

    let maximum_pressure_v1 = compute_maximum_pressure_v1(&valves, &distance_matrix, &non_zero_valves, start_at);

    let maximum_pressure_v2 = compute_maximum_pressure_v2(&valves, &distance_matrix, &non_zero_valves, start_at);

    (maximum_pressure_v1, maximum_pressure_v2)
}


fn get_combinations(values: &Vec<usize>, n: usize) -> Vec<(Vec<usize>, Vec<usize>)> {
    let mut combinations = Vec::new();

    for combination in values.clone().into_iter().combinations(n) {
        let mut remaining_values = values.clone();
        for value in combination.clone() {
            remaining_values.retain(|&v| v != value);
        }
        combinations.push((combination, remaining_values));
    }

    combinations
}

fn main() {
    println!("Starting execution!\n");
    let start = Instant::now();

    let input = "./input.txt";
    
    let (solution_v1, solution_v2) = solution(input);

    println!("V1 solution is {}", solution_v1);
    println!("V1 solution is {}", solution_v2);
    

    let duration = start.elapsed();
    println!("\nTime elapsed is: {:?}", duration);
}
