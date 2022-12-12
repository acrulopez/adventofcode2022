use core::panic;
use std::time::Instant;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_input(input: &str) -> Vec<(char, u8)>  {
    let lines = read_lines(input)
        .unwrap_or_else(|_| panic!("File '{input}' should be accesible"));

    let mut actions: Vec<(char, u8)> = Vec::new();
    let mut direction_steps;

    for line in lines {
        if let Ok(line) = line {
            direction_steps = line.split_whitespace();
            actions.push(
                (direction_steps.next().unwrap().parse().unwrap(), 
                direction_steps.next().unwrap().parse().unwrap())
            );
        }
    }

    actions
}

fn move_head(
    mut head_coords: (i32, i32),
    mut tail_coords: (i32, i32),
    direction:char, 
    visited_coords:&mut HashSet<(i32,i32)>) 
    -> ((i32, i32), (i32, i32)) {

    match direction {
        'U' => head_coords.1 += 1,
        'D' => head_coords.1 -= 1,
        'R' => head_coords.0 += 1,
        'L' => head_coords.0 -= 1,
        _ => panic!("The options are only U, D, R and L.")
    }
    
    let dist_x = head_coords.0 - tail_coords.0;
    let dist_y = head_coords.1 - tail_coords.1;

    if dist_x.abs() == 2 {
        tail_coords.1 += dist_y;
        if dist_x == 2 {
            tail_coords.0 += 1
        }
        else {
            tail_coords.0 -= 1
        }
    }
    else if dist_y.abs() == 2 {
        tail_coords.0 += dist_x;
        if dist_y == 2 {
            tail_coords.1 += 1
        }
        else {
            tail_coords.1 -= 1
        }
    }

    visited_coords.insert(tail_coords);

    (head_coords, tail_coords)

}

fn move_head_v2(
    last_coords: (i32,i32),
    mut current_coords: (i32, i32)
) -> (i32, i32) {

    
    let dist_x = last_coords.0 - current_coords.0;
    let dist_y = last_coords.1 - current_coords.1;

    if dist_x.abs() == 2 && dist_y.abs() == 2 {
        current_coords.0 += dist_x.clamp(-1, 1);
        current_coords.1 += dist_y.clamp(-1, 1);
    }
    else if dist_x.abs() == 2 {
        current_coords.1 += dist_y;
        if dist_x == 2 {
            current_coords.0 += 1
        }
        else {
            current_coords.0 -= 1
        }
    }
    else if dist_y.abs() == 2 {
        current_coords.0 += dist_x;
        if dist_y == 2 {
            current_coords.1 += 1
        }
        else {
            current_coords.1 -= 1
        }
    }
    
    current_coords

}

fn get_visited_positions(input: &str) -> usize {

    let actions = parse_input(input);
    let mut visited_coords = HashSet::new();
    visited_coords.insert((0,0));

    let mut head_coords:(i32, i32) = (0,0);
    let mut tail_coords:(i32, i32) = (0,0);

    for (direction, times) in actions {
        for _ in 0..times {
        (head_coords, tail_coords) = move_head(
            head_coords, 
            tail_coords, 
            direction, 
            &mut visited_coords);
        }
    }


    return visited_coords.len();
    
}

fn get_visited_positions_v2(input: &str) -> usize {
    const ROPE_LEN:usize = 10;

    let actions = parse_input(input);
    let mut visited_coords = HashSet::new();
    visited_coords.insert((0,0));

    let mut rope_coords:[(i32,i32);10] = [(0,0); ROPE_LEN];

    for (direction, times) in actions {
        for _ in 0..times {
            // Move head
            match direction {
                'U' => rope_coords[0].1 -= 1,
                'D' => rope_coords[0].1 += 1,
                'R' => rope_coords[0].0 += 1,
                'L' => rope_coords[0].0 -= 1,
                _ => panic!("The options are only U, D, R and L.")
            }

            for i in 1..rope_coords.len() {
                rope_coords[i] = move_head_v2(
                rope_coords[i-1], 
                rope_coords[i]);
            }



            visited_coords.insert(rope_coords[ROPE_LEN-1]);
        }
    }


    return visited_coords.len();
    
}


fn main() {
    println!("Starting execution!\n");
    let start = Instant::now();

    let input = "./input.txt";
    
    let visited_positions = get_visited_positions(input);
    let visited_positions_v2 = get_visited_positions_v2(input);

    println!("The number of visited positions is {visited_positions}");
    println!("The number of visited positions on v2 is {visited_positions_v2}");

    let duration = start.elapsed();
    println!("\nTime elapsed is: {:?}", duration);
}
