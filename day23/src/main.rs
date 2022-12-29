use std::collections::{HashSet, HashMap};
use std::time::Instant;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const DIRECTIONS: [[(isize, isize); 3]; 4] = [[(-1,0), (-1,1), (-1,-1)], [(1,0), (1,1), (1,-1)], [(0,-1), (1,-1), (-1,-1)], [(0,1), (1,1), (-1,1)]];
const ITER_V1:usize = 10;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_input(input: &str) -> HashSet<(isize, isize)> {
    let lines = read_lines(input)
        .unwrap_or_else(|_| panic!("File '{input}' should be accesible"));


    let elves_positions: HashSet<(isize, isize)> = lines.enumerate()
        .flat_map(|(x, line)| line.unwrap().chars()
        .enumerate().filter_map(|(y, ch)| if ch == '#' {return Some((x as isize,y as isize))} else {return None})
        .collect::<Vec<(isize, isize)>>())
        .collect();


    elves_positions
}

fn sum_coordinates(coord1: &(isize, isize), coord2: &(isize, isize)) -> (isize, isize) {
    (coord1.0 + coord2.0, coord1.1 + coord2.1)
} 

fn is_elf_alone(elves_positions: &HashSet<(isize, isize)>, elf_pos: (isize, isize)) -> bool {
    for x in -1..=1 {
        for y in -1..=1 {
            if x != y || x != 0 {
                if elves_positions.contains(&sum_coordinates(&elf_pos, &(x, y))) {
                    return false;
                }
            }
        }
    }
    true
}

fn solution_v1(input: &str) -> isize {
    let mut elves_positions = parse_input(input);

    let mut directions;
    let mut current_dir = 0;
    let mut elves_movements = HashMap::new();
    for _ in 0..ITER_V1 {

        for pos in elves_positions.iter() {
            if is_elf_alone(&elves_positions, *pos) {
                continue;
            }
            'directions: for dir_idx in 0..4 {
                directions = DIRECTIONS[(dir_idx + current_dir) % 4];

                // Check if the three spots are free, if not, check the next direction
                for direction in directions {
                    if elves_positions.contains(&sum_coordinates(pos, &direction)) {
                        continue 'directions;
                    }
                }
                    
                // If there is already one elf wanting to go there, none will move
                let new_pos = sum_coordinates(pos, &directions[0]);
                if elves_movements.contains_key(&new_pos) {
                    elves_movements.insert(new_pos, None);
                }
                // Insert the elve that wants to move there
                else {
                    elves_movements.insert(new_pos, Some(pos.clone()));
                }

                break 'directions;
                
            }
        }

        for (new_pos, prev_pos) in elves_movements.drain() {
            if let Some(prev_pos) = prev_pos {
                elves_positions.remove(&prev_pos);
                elves_positions.insert(new_pos);
            }
        }

        current_dir = (current_dir + 1) % 4;
    }

    let min_x = elves_positions.iter().map(|(x, _)| x).min().unwrap();
    let max_x = elves_positions.iter().map(|(x, _)| x).max().unwrap();
    let min_y = elves_positions.iter().map(|(_, y)| y).min().unwrap();
    let max_y = elves_positions.iter().map(|(_, y)| y).max().unwrap();
    
    (1+max_x-min_x) * (1+max_y-min_y) - elves_positions.len() as isize
}

fn solution_v2(input: &str) -> isize {
    let mut elves_positions = parse_input(input);

    let mut directions;
    let mut current_dir = 0;
    let mut elves_movements = HashMap::new();
    let mut finish = false;
    let mut iterations = 0;
    while !finish {
        iterations += 1;
        finish = true;
        for pos in elves_positions.iter() {
            if is_elf_alone(&elves_positions, *pos) {
                continue;
            }
            'directions: for dir_idx in 0..4 {
                directions = DIRECTIONS[(dir_idx + current_dir) % 4];

                // Check if the three spots are free, if not, check the next direction
                for direction in directions {
                    if elves_positions.contains(&sum_coordinates(pos, &direction)) {
                        continue 'directions;
                    }
                }
                    
                // If there is already one elf wanting to go there, none will move
                let new_pos = sum_coordinates(pos, &directions[0]);
                if elves_movements.contains_key(&new_pos) {
                    elves_movements.insert(new_pos, None);
                }
                // Insert the elve that wants to move there
                else {
                    elves_movements.insert(new_pos, Some(pos.clone()));
                }

                break 'directions;
                
            }
        }

        for (new_pos, prev_pos) in elves_movements.drain() {
            if let Some(prev_pos) = prev_pos {
                elves_positions.remove(&prev_pos);
                elves_positions.insert(new_pos);
                finish = false;
            }
        }

        current_dir = (current_dir + 1) % 4;
    }

    iterations

}


fn main() {

    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        panic!("Should provide exactly one argument");
    }
    let input = &args[1];
    // let input = "input.txt";

    // --------------------------V1--------------------------------

    println!("Starting execution version 1!\n");
    let start = Instant::now();
    
    let solution_v1 = solution_v1(input);
    println!("V1 solution is {}", solution_v1);

    let duration = start.elapsed();
    println!("\nTime elapsed is: {:?}", duration);

    // --------------------------V"--------------------------------

    println!("Starting execution version 1!\n");
    let start = Instant::now();
    
    let solution_v2 = solution_v2(input);
    println!("V2 solution is {}", solution_v2);

    let duration = start.elapsed();
    println!("\nTime elapsed is: {:?}", duration);
    
}
