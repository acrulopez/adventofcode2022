use std::collections::{HashSet, VecDeque};
use std::time::Instant;
use std::{env, usize, vec};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const INITIAL_POSITION: (usize, usize) = (usize::MAX, usize::MAX);


#[derive(Debug, Clone, Copy)]
enum Blizzard {
    Upwards,
    Downwards,
    Rightwards,
    Leftwards
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn char_into_blizzard(ch: char) -> Option<Vec<Blizzard>> {
    match ch {
        '.' => Some(vec![]),
        '<' => Some(vec![Blizzard::Leftwards]),
        '>' => Some(vec![Blizzard::Rightwards]),
        '^' => Some(vec![Blizzard::Upwards]),
        'v' => Some(vec![Blizzard::Downwards]),
        _ => None
    }
}

fn parse_input(input: &str) -> Vec<Vec<Vec<Blizzard>>> {
    let mut lines = read_lines(input)
        .unwrap_or_else(|_| panic!("File '{input}' should be accesible"));


    lines.next(); // Remove first line
    let mut grid: Vec<Vec<Vec<Blizzard>>>= lines
        .map(|line| line.unwrap().chars()
        .filter_map(|x|char_into_blizzard(x))
        .collect())
        .collect();
    grid.pop();


    grid
}

fn blizzard_new_position(
    grid_dimensions: &(usize, usize), 
    current_pos: &(usize, usize), 
    blizzard: &Blizzard) -> (usize, usize) {
    
    match blizzard {
        Blizzard::Downwards => ((current_pos.0 + 1) % grid_dimensions.0, current_pos.1),
        Blizzard::Rightwards => (current_pos.0, (current_pos.1 + 1) % grid_dimensions.1),
        Blizzard::Upwards => {
            if current_pos.0 != 0 {
                return (current_pos.0 - 1, current_pos.1);
            }
            else {
                return (grid_dimensions.0 - 1, current_pos.1);
            }
        },
        Blizzard::Leftwards => {
            if current_pos.1!= 0 {
                return (current_pos.0, current_pos.1 - 1);
            }
            else {
                return (current_pos.0, grid_dimensions.1 - 1);
            }
        },
    }

}

fn get_possible_elf_movements(
    grid: &Vec<Vec<Vec<Blizzard>>>, 
    elf_position: &(usize, usize),
    grid_dimensions: &(usize, usize)) -> Vec<(usize, usize)> {

    if *elf_position == INITIAL_POSITION {
        if grid[0][0].len() == 0 {
            return vec![(0,0), INITIAL_POSITION];
        }
        return vec![INITIAL_POSITION];
    }
    else if elf_position == grid_dimensions {
        if grid[grid_dimensions.0-1][grid_dimensions.0-1].len() == 0 {
            return vec![(grid_dimensions.0 - 1, grid_dimensions.1 - 1), *elf_position];
        }
        return vec![*elf_position];
    }

    let mut possible_movements = vec![];

    // North
    if elf_position.0 != 0 && grid[elf_position.0 - 1][elf_position.1].len() == 0 {
        possible_movements.push((elf_position.0 - 1, elf_position.1));
    }
    // South
    if elf_position.0 != grid_dimensions.0 - 1 && grid[elf_position.0 + 1][elf_position.1].len() == 0 {
        possible_movements.push((elf_position.0 + 1, elf_position.1));
    }
    // Left
    if elf_position.1 != 0 && grid[elf_position.0][elf_position.1 - 1].len() == 0 {
        possible_movements.push((elf_position.0, elf_position.1 - 1));
    }
    // Right
    if elf_position.1 != grid_dimensions.1 - 1 && grid[elf_position.0][elf_position.1 + 1].len() == 0 {
        possible_movements.push((elf_position.0, elf_position.1 + 1));
    }
    // Same position
    if grid[elf_position.0][elf_position.1].len() == 0 {
        possible_movements.push((elf_position.0, elf_position.1));
    }

    possible_movements

}

fn lowest_common_multple(n1: usize, n2: usize) -> usize {
    let mut x;
    let mut y;
    if n1 > n2 {
        x = n1;
        y = n2;
    }
    else {
        x = n2;
        y = n1;
    }

    let mut rem = x % y;

    while rem != 0 {
        x = y;
        y = rem;
        rem = x % y;
    }

    return (n1 * n2) / y;
}

fn get_quickest_path(grid: &mut Vec<Vec<Vec<Blizzard>>>, 
    initial_pos: (usize, usize), 
    final_pos: (usize, usize)) -> isize {
        
    let grid_dimensions = (grid.len(), grid[0].len());
    // println!("{grid_dimensions:?}");
    let lcm = lowest_common_multple(grid_dimensions.0, grid_dimensions.1);
    let initial_time: usize = 0;
    let mut grid_time = 0;

    let mut blizzards_new_pos = vec![];
    let mut position_queue = VecDeque::new();
    let mut visited_cells_map = HashSet::new();

    position_queue.push_front((initial_pos, initial_time));

    while let Some((elf_position, time)) = position_queue.pop_front()  {


        // Check if we are in the final step
        if elf_position.0 == final_pos.0 && elf_position.1 == final_pos.1 {
            return (time+1) as isize;
        }

        // If that cell was visited on a spot in which the map is the same (lcm multiple), we skip it
        if (lcm < time && visited_cells_map.contains(&(elf_position, time-lcm))) 
            || visited_cells_map.contains(&(elf_position, time)) {

            visited_cells_map.insert((elf_position, time));
            continue;
        }


        if grid_time <= time {

            // We empty the grid and fill the vector 'blizzards_new_pos' with the new positions of the blizzards
            for (x, blizzard_row) in grid.iter_mut().enumerate() {
                for (y, blizzard_vec) in blizzard_row.iter_mut().enumerate() {
                    for blizzard in blizzard_vec.drain(..) {
                        // println!("{:?}, {:?}, {:?}", &(x, y), blizzard, blizzard_new_position(&grid_dimensions, &(x, y), &blizzard));
                        blizzards_new_pos.push((blizzard, blizzard_new_position(&grid_dimensions, &(x, y), &blizzard)));
                    }
                }
            }

            // println!("{blizzards_new_pos:?}\n\n");

            // We empty the vector 'blizzards_new_pos' and fill the grid with the new positions
            for (blizard, pos) in blizzards_new_pos.drain(..) {
                grid[pos.0][pos.1].push(blizard);
            }
            grid_time += 1;

        }       
        
        // println!("{:?}", grid[0][0]);
        // println!("{:?} {time}", elf_position);

        // We insert the new positions on the queue and the set
        let possible_elf_movements = get_possible_elf_movements(&grid, &elf_position, &grid_dimensions);

        position_queue.extend(possible_elf_movements.iter().map(|x| (*x, time+1)));
        visited_cells_map.insert((elf_position, time));
    }

    -1

}

fn solution(input: &str) -> (isize, isize) {
    let mut grid = parse_input(input);
    let grid_dimensions = (grid.len(), grid[0].len());

    let quickest_path = get_quickest_path(&mut grid, INITIAL_POSITION, (grid_dimensions.0-1, grid_dimensions.1-1));
    let quickest_path_2 = get_quickest_path(&mut grid, grid_dimensions, (0,0));
    let quickest_path_3 = get_quickest_path(&mut grid, INITIAL_POSITION, (grid_dimensions.0-1, grid_dimensions.1-1));

    (quickest_path, quickest_path+quickest_path_2+quickest_path_3)
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
    
    let (solution_v1, solution_v2) = solution(input);
    println!("V1 solution is {}", solution_v1);
    println!("V2 solution is {}", solution_v2);

    let duration = start.elapsed();
    println!("\nTime elapsed is: {:?}", duration);

}

// for x in 0..grid.len() {
//     for y in 0..grid[0].len() {
//         print!("{:?}\t\t", grid[x][y]);
//     }
//     println!("");
// }