use std::collections::HashMap;
use std::time::Instant;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::hash_map::{DefaultHasher, Entry};
use std::hash::{Hash, Hasher};

const CAVE_WIDTH:usize = 7;
const ROCK_NUMBER_V1: usize = 2022;
const ROCK_NUMBER_V2: usize = 1000000000000;


#[derive(Debug)]
enum Direction {
    Right,
    Left
}

struct RockProperties {
    starting_position: (usize, usize),
    left_right_bounds: (usize, usize),
    rock_positions: Vec<(isize, isize)>,
    rock_height: usize,
}

enum RockType {
    HorizontalLine(RockProperties),
    Plus(RockProperties),
    InvertedL(RockProperties),
    VerticalLine(RockProperties),
    Squared(RockProperties)
}
const N_ROCK_TYPES: usize = 5;


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_input(input: &str) -> Vec<Direction> {
    let lines = read_lines(input)
        .unwrap_or_else(|_| panic!("File '{input}' should be accesible"));

    let mut gas_directions = vec![];
    for line in lines {
        if let Ok(line) = line {
            for ch in line.chars() {
                if ch == '<' {gas_directions.push(Direction::Left)}
                else {gas_directions.push(Direction::Right)}
            }
        }
    }

    gas_directions
}

fn create_rock_types() -> [RockType; N_ROCK_TYPES] {
    let rock_types: [RockType; N_ROCK_TYPES] = [
        RockType::HorizontalLine(RockProperties { 
            starting_position: (2,3),
            left_right_bounds: (0,3),
            rock_positions: vec![(0,0), (1,0), (2,0), (3,0)],
            rock_height: 1
        }), 
        RockType::Plus(RockProperties {  
            starting_position: (3,3),
            left_right_bounds: (1,5),
            rock_positions: vec![(0,0), (0,1), (0,2), (-1,1),(1,1)],
            rock_height: 3
        }), 
        RockType::InvertedL(RockProperties {  
            starting_position: (2,3),
            left_right_bounds: (0,4),
            rock_positions: vec![(0,0), (1,0), (2,0), (2,1),(2,2)],
            rock_height: 3
        }), 
        RockType::VerticalLine(RockProperties {  
            starting_position: (2,3),
            left_right_bounds: (0,6),
            rock_positions: vec![(0,0), (0,1), (0,2), (0,3)],
            rock_height: 4
        }), 
        RockType::Squared(RockProperties {  
            starting_position: (2,3),
            left_right_bounds: (0,5),
            rock_positions: vec![(0,0), (0,1), (1,0), (1,1)],
            rock_height: 2
        }),
    ];
    rock_types
}


fn increase_cave_ceiling(grid: &mut Vec<Vec<bool>>, height:usize) {
    let extension = 7 + height - grid[0].len();
    if extension > 0 {
        for v in grid {
            v.extend(vec![false; extension]);
        }
    }
}

fn can_move(grid: &Vec<Vec<bool>>, rock_properties:&RockProperties, rock_position: (usize, usize), direction: Direction) -> bool {

    match direction {
        Direction::Left => {
            if rock_position.0 > rock_properties.left_right_bounds.0 {
                for rock_relative_position in &rock_properties.rock_positions {
                    let (x, y) = rock_relative_position;
                    if grid[(rock_position.0 as isize + x - 1) as usize]
                            [(rock_position.1 as isize + y) as usize] {
                        return false
                    }
                }
            }
            else {return false;}
        }
        Direction::Right => {
            if rock_position.0 < rock_properties.left_right_bounds.1 {
                for rock_relative_position in &rock_properties.rock_positions {
                    let (x, y) = rock_relative_position;
                    if grid[(rock_position.0 as isize + x + 1) as usize]
                            [(rock_position.1 as isize + y) as usize] {
                        return false
                    }
                }
            }
            else {return false;}
        }
    }

    true
}

fn solution_v1(input: &str) -> usize {

    let gas_directions = parse_input(input);
    let mut grid: Vec<Vec<bool>> = vec![vec![]; CAVE_WIDTH];

    let mut gas_idx = 0;
    let mut height = 0;
    increase_cave_ceiling(&mut grid, height);

    let mut current_rock_type;
    let rock_types = create_rock_types();
    for resting_rocks in 0..ROCK_NUMBER_V1 {
        current_rock_type = &rock_types[resting_rocks%N_ROCK_TYPES];

        let mut rock_position;

        match current_rock_type {
            RockType::HorizontalLine(rock_properties) |
            RockType::Plus(rock_properties) |
            RockType::InvertedL(rock_properties) |
            RockType::VerticalLine(rock_properties) |
            RockType::Squared(rock_properties) => {

                rock_position = (rock_properties.starting_position.0, rock_properties.starting_position.1+height);
                'rock_falling: loop {
                    match gas_directions[gas_idx%gas_directions.len()] {
                        Direction::Left => {
                            if can_move(&grid, rock_properties, rock_position, Direction::Left) {rock_position.0 -= 1;}
                        }
                        Direction::Right => {
                            if can_move(&grid, rock_properties, rock_position, Direction::Right) {rock_position.0 += 1;}
                        } 
                    }
                    gas_idx += 1;

                    

                    for rock_relative_position in &rock_properties.rock_positions {
                        let (x, y) = rock_relative_position;
                        if rock_position.1 == 0 || grid[(rock_position.0 as isize + x) as usize][(rock_position.1 as isize + y - 1) as usize] {
                            
                            // The rock has found a place to stay

                            if rock_position.1 + rock_properties.rock_height > height {
                                height = rock_position.1 + rock_properties.rock_height;
                            }
                            
                            increase_cave_ceiling(&mut grid, height);

                            // We add the rock to the grid
                            for rock_relative_position in &rock_properties.rock_positions {
                                let (x, y) = rock_relative_position;
                                grid[(rock_position.0 as isize +x) as usize][(rock_position.1 as isize +y) as usize] = true;
                            }

                            break 'rock_falling;
                        }
                    }
                    rock_position.1 -= 1;
                }
            }
        }
    }
    height
}


fn calculate_hash(grid: &Vec<Vec<bool>>, gas_idx: usize) -> u64 {
    let len = grid[0].len();
    let mut s = DefaultHasher::new();
    if len > 20 {
        for v in grid {
            v[len-20..].hash(&mut s);
        }
    }
    gas_idx.hash(&mut s);
    // t.hash(&mut s);
    s.finish()
}


fn solution_v2(input: &str) -> usize {

    let gas_directions = parse_input(input);
    let mut grid: Vec<Vec<bool>> = vec![vec![]; CAVE_WIDTH];
    let mut grid_and_gas_snapshot = HashMap::new();
    let mut computed_height_by_repetition = 0;

    let mut gas_idx = 0;
    let mut height = 0;
    increase_cave_ceiling(&mut grid, height);

    let mut current_rock_type;
    let rock_types = create_rock_types();
    let mut resting_rocks = 0;
    while resting_rocks < ROCK_NUMBER_V2 {
        current_rock_type = &rock_types[resting_rocks%N_ROCK_TYPES];

        let mut rock_position;

        match current_rock_type {
            RockType::HorizontalLine(rock_properties) |
            RockType::Plus(rock_properties) |
            RockType::InvertedL(rock_properties) |
            RockType::VerticalLine(rock_properties) |
            RockType::Squared(rock_properties) => {

                rock_position = (rock_properties.starting_position.0, rock_properties.starting_position.1+height);
                'rock_falling: loop {
                    match gas_directions[gas_idx%gas_directions.len()] {
                        Direction::Left => {
                            if can_move(&grid, rock_properties, rock_position, Direction::Left) {rock_position.0 -= 1;}
                        }
                        Direction::Right => {
                            if can_move(&grid, rock_properties, rock_position, Direction::Right) {rock_position.0 += 1;}
                        } 
                    }
                    gas_idx += 1;

                    

                    for rock_relative_position in &rock_properties.rock_positions {
                        let (x, y) = rock_relative_position;
                        if rock_position.1 == 0 || grid[(rock_position.0 as isize + x) as usize][(rock_position.1 as isize + y - 1) as usize] {
                            
                            // The rock has found a place to stay

                            if rock_position.1 + rock_properties.rock_height > height {
                                height = rock_position.1 + rock_properties.rock_height;
                            }
                            
                            increase_cave_ceiling(&mut grid, height);

                            // We add the rock to the grid
                            for rock_relative_position in &rock_properties.rock_positions {
                                let (x, y) = rock_relative_position;
                                grid[(rock_position.0 as isize +x) as usize][(rock_position.1 as isize +y) as usize] = true;
                            }

                            resting_rocks += 1;
                            break 'rock_falling;
                        }
                    }
                    rock_position.1 -= 1;
                }
            }
        }
        match grid_and_gas_snapshot.entry(calculate_hash(&grid, gas_idx%gas_directions.len())) {
            Entry::Occupied(o) => {
                let (prev_height, prev_n_rock) = o.get();                

                let height_diff = height - prev_height;
                let n_rock_diff = resting_rocks - prev_n_rock;

                let rock_repeats = (ROCK_NUMBER_V2 - resting_rocks) / n_rock_diff;
                resting_rocks += rock_repeats * n_rock_diff;
                computed_height_by_repetition += rock_repeats * height_diff;

            },
            Entry::Vacant(v) => {
                v.insert((height, resting_rocks));
            },
        };
    }

    height + computed_height_by_repetition
}

fn solution(input: &str) -> (usize, usize) {

    return (solution_v1(input), solution_v2(input));
}

fn main() {
    println!("Starting execution!\n");
    let start = Instant::now();

    let input = "./input.txt";
    
    let (solution_v1, solution_v2) = solution(input);

    println!("V1 solution is {}", solution_v1);
    println!("V2 solution is {}", solution_v2);
    

    let duration = start.elapsed();
    println!("\nTime elapsed is: {:?}", duration);
}
