use std::time::Instant;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

const STARTING_DIRECTION: Direction = Direction::Right;

enum GridObject {
    Wall,
    Air,
    Nothing
}

enum Action {
    Movement(u32),
    Turn(char)
}

#[repr(u8)] #[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl From<u8> for Direction {
    fn from(orig: u8) -> Self {
        match orig {
            0 => return Direction::Right,
            1 => return Direction::Down,
            2 => return Direction::Left,
            3 => return Direction::Up,
            _ => panic!("should be 0, 1, 2 or 3")
        };
    }
}

struct GridBounds {
    left: Vec<usize>,
    right: Vec<usize>,
    upper: Vec<usize>,
    bottom: Vec<usize>,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_input(input: &str) -> (Vec<Vec<GridObject>>, Vec<Action> ) {
    let lines = read_lines(input)
        .unwrap_or_else(|_| panic!("File '{input}' should be accesible"));

    
    let re = Regex::new(r"([0-9]+)|([LR]+)").unwrap();

    // Get the grid and the directions as separate objects
    let mut grid: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let directions = grid.pop().unwrap();
    grid.pop(); // Remove the empty line

    // Parse grid
    let grid: Vec<Vec<GridObject>> = grid.iter()
        .map(|x| x
        .chars()
        .map(|x| if x == '#' {GridObject::Wall} else if x == '.' {GridObject::Air} else {GridObject::Nothing})
        .collect())
        .collect();

    // Parse directions
    fn parse_direction(dir_str: &str) -> Action {
        let dir_num = dir_str.parse::<u32>();
        if let Ok(dir_num) = dir_num {
            return Action::Movement(dir_num);
        }
        return Action::Turn(dir_str.chars().next().unwrap());
    }

    let directions: Vec<Action> = re
    .find_iter(&directions)
    .map(|x| parse_direction(x.as_str()))
    .collect();


    (grid, directions)
}

fn get_bounds(grid: &Vec<Vec<GridObject>>) -> GridBounds {
    // Left bounds
    let left_bounds: Vec<usize> = grid.iter()
    .map(|x| x.iter()
    .position(|x| if let GridObject::Nothing = x {false} else {true}).unwrap())
    .collect();

    // Right bounds
    let right_bounds:Vec<usize> = grid.iter().map(|x| x.len()).collect();

    // Upper bounds
    let max_len = grid.iter().max_by_key(|x| x.len()).unwrap().len();
    let upper_bounds: Vec<usize> = (0..max_len)
        .map(|it| left_bounds.iter().zip(&right_bounds)
        .position(|(left, right)| it >= *left && it < *right).unwrap())
        .collect();

    // Bottom bounds
    let bottom_bounds: Vec<usize> = (0..max_len)
        .map(|it| left_bounds.iter().zip(&right_bounds)
        .rev()
        .position(|(left, right)| it >= *left && it < *right).unwrap())
        .map(|x| grid.len()-x)
        .collect();

    GridBounds {left: left_bounds, right: right_bounds, upper: upper_bounds, bottom: bottom_bounds}

}


fn get_new_position(
    grid: &Vec<Vec<GridObject>>, 
    bounds: &GridBounds, 
    mut current_position: (usize, usize),
    current_direction: &Direction) 
-> Option<(usize, usize)> {

    match current_direction {
        Direction::Down => {
            if bounds.bottom[current_position.1] > current_position.0 + 1 {
                current_position.0 += 1;
            }
            else {
                current_position.0 = bounds.upper[current_position.1];
            }
        },
        Direction::Up => {
            if bounds.upper[current_position.1] < current_position.0 {
                current_position.0 -= 1;
            }
            else {
                current_position.0 = bounds.bottom[current_position.1] - 1;
            }
        },
        Direction::Right => {
            if bounds.right[current_position.0] > current_position.1 + 1 {
                current_position.1 += 1;
            }
            else {
                current_position.1 = bounds.left[current_position.0];
            }
        },
        Direction::Left => {
            if bounds.left[current_position.0] < current_position.1 {
                current_position.1 -= 1;
            }
            else {
                current_position.1 = bounds.right[current_position.0] - 1;
            }
        },

    }

    if let GridObject::Wall = grid[current_position.0][current_position.1] {
        return None;
    }
    return Some(current_position);

}

fn final_position(grid: &Vec<Vec<GridObject>>, actions: &Vec<Action>, bounds: &GridBounds) -> ((usize, usize), Direction) {

    let starting_point: (usize, usize) = (0, bounds.left[0]);
    let mut current_position = starting_point;
    let mut current_direction = STARTING_DIRECTION;

    for action in actions {
        match action {
            Action::Movement(n) => {
                for _ in 0..*n {
                    if let Some(new_pos) = get_new_position(grid, bounds, current_position, &current_direction) {
                        current_position = new_pos;
                    }
                    else {
                        break;
                    }
                }
            }
            Action::Turn(ch) => {
                if *ch == 'R' {
                    current_direction = Direction::from((current_direction as u8 + 1).rem_euclid(4));
                }
                else {
                    current_direction = Direction::from(((current_direction as i8 - 1) as u8).rem_euclid(4));
                }
            }
        }
    }

    (current_position, current_direction)
}

fn solution_v1(input: &str) -> usize {
    let (grid, actions) = parse_input(input);

    let bounds = get_bounds(&grid);

    let (final_position, final_direction) = final_position(&grid, &actions, &bounds);

    println!("{:?} {:?}", final_position, final_direction);

    (1000 * (final_position.0+1)) + (4 * (final_position.1+1)) + (final_direction as usize)
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
    
}
