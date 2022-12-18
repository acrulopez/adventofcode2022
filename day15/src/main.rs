use std::time::Instant;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;
use std::cmp::{min, max};

const MAX_POSITION: i32 = 4000000;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn merge_intervals(mut intervals: Vec<(i32,i32)>) -> Vec<(i32,i32)> {

    intervals.sort();

    let mut merged_intervals = vec![intervals[0]];

    for interval in intervals {
        if merged_intervals.last().unwrap().1 >= interval.0 {
            let last_idx = merged_intervals.len()-1;
            if  merged_intervals[last_idx].1 < interval.1 {
                merged_intervals[last_idx].1 = interval.1;
            }
        }
        else {
            merged_intervals.push(interval);
        }
    }

    merged_intervals
}

fn get_non_beacon_positions(input: &str, row: i32) -> (i32, i128) {
    let lines = read_lines(input)
        .unwrap_or_else(|_| panic!("File '{input}' should be accesible"));

    let number_re = Regex::new(r"-?\d+").unwrap();

    // For part 1
    let mut intervals = vec![];

    // For Part 2
    // let mut possible_beacon_grid = [[true; MAX_POSITION as usize]; MAX_POSITION as usize];
    // let mut possible_beacon_grid: [Vec<(i32, i32)>; MAX_POSITION as usize] = [vec![].clone(); MAX_POSITION as usize];
    let mut possible_beacon_grid: Vec<Vec<(i32,i32)>> = vec![];
    for _ in 0..MAX_POSITION {possible_beacon_grid.push(vec![])}



    for line in lines {
        if let Ok(line) = line {
            
            let coordinates = number_re.
                find_iter(&line).filter_map(|number| number.as_str().parse::<i32>().ok())
                .collect::<Vec<i32>>();

            // Part 1

            let distance_to_beacon = (coordinates[0] - coordinates[2]).abs() + (coordinates[1] - coordinates[3]).abs();
            let distance_to_row = (coordinates[1] - row).abs();

            if distance_to_beacon > distance_to_row {
                let diff_distances = distance_to_beacon - distance_to_row;
                if coordinates[3] != row {
                    intervals.push((coordinates[0] - diff_distances, coordinates[0] + diff_distances));
                }
                else {
                    if coordinates[0] > coordinates[2] {
                        intervals.push((coordinates[0] - diff_distances + 1, coordinates[0] + diff_distances));
                    }
                    else {
                        intervals.push((coordinates[0] - diff_distances, coordinates[0] + diff_distances - 1));
                    }
                }
            }

            // Part 2
            for p in 0..distance_to_beacon+1 {
                let q = (distance_to_beacon-p).abs();
                let idx_minus = coordinates[1] - q;
                let idx_plus = coordinates[1] + q;

                let interval_min = max(coordinates[0]-p,0);
                let interval_max = min(MAX_POSITION ,coordinates[0]+p);

                if interval_min <= interval_max {
                    if idx_minus >= 0 && idx_minus < MAX_POSITION {
                        possible_beacon_grid[idx_minus as usize].push((interval_min, interval_max));
                    }
                    if idx_plus >= 0 && idx_plus < MAX_POSITION {
                        possible_beacon_grid[idx_plus as usize].push((interval_min, interval_max));
                    }
                }
                
            }
            
        }

    }

    // Wrap up part 1
    let mut non_beacon_positions = 0;
    let merged_intervals = merge_intervals(intervals);
    for interval in merged_intervals {
        non_beacon_positions += interval.1 - interval.0 + 1;
    }

    // Wrap up part 2
    let mut tuning_frequence = 0;
    for (x, grid_intervals) in possible_beacon_grid.into_iter().enumerate() {
        let merged_intervals = merge_intervals(grid_intervals);
        if merged_intervals.len() > 1 {
            tuning_frequence = (((merged_intervals[0].1 + merged_intervals[1].0)/2) as i128 * MAX_POSITION as i128) + x as i128;
            break;
        }
    }
    
    (non_beacon_positions, tuning_frequence)
}

fn main() {
    println!("Starting execution!\n");
    let start = Instant::now();

    let input = "./input.txt";
    
    let (non_beacon_positions, tuning_frequence) = get_non_beacon_positions(input, 2000000);

    println!("The number of non beacon positions is {}", non_beacon_positions);
    println!("The tuning frequence is {}", tuning_frequence);
    

    let duration = start.elapsed();
    println!("\nTime elapsed in expensive_function() is: {:?}", duration);
}
