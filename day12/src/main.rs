use std::time::Instant;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

enum Direction {
    Top,
    Bottom,
    Right,
    Left,
}
const DIRECTIONS: [Direction; 4] = [Direction::Bottom, Direction::Top, Direction::Right, Direction::Left];

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, (usize, usize), (usize,usize), Vec<(usize, usize)>)   {
    let lines = read_lines(input)
        .unwrap_or_else(|_| panic!("File '{input}' should be accesible"));

    let mut grid: Vec<Vec<char>> = vec![];
    let mut start_position = None;
    let mut end_position = None;
    let mut minimum_height_cells: Vec<(usize, usize)> = vec![];

    for (i, line) in lines.enumerate() {
        grid.push(vec![]);
        if let Ok(line) = line {
            for (j, ch) in line.chars().enumerate() {
                if ch == 'S' {
                    start_position = Some((i,j));
                    minimum_height_cells.push((i,j));
                    grid[i].push('a');
                }
                else if ch == 'E' {
                    end_position = Some((i,j));
                    grid[i].push('z');
                }
                else {
                    if ch == 'a' {minimum_height_cells.push((i,j))}
                    grid[i].push(ch);
                }
            }
        }
    }

    (grid, start_position.unwrap(), end_position.unwrap(), minimum_height_cells)
}

fn move_to_cell(current_cell: (usize, usize), direction:Direction, char_grid: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    match direction {
        Direction::Bottom => {
            if current_cell.0 < char_grid.len()-1 {Some((current_cell.0+1, current_cell.1))}
            else {None}
        },
        Direction::Top => {
            if current_cell.0 != 0 {Some((current_cell.0-1, current_cell.1))}
            else {None}
        },
        Direction::Right => {
            if current_cell.1 < char_grid[0].len()-1 {Some((current_cell.0, current_cell.1+1))}
            else {None}
        },
        Direction::Left => {
            if current_cell.1 != 0 {Some((current_cell.0, current_cell.1-1))}
            else {None}
        },
        // _ => panic!("Should be Top, Bottom, Right or Left")
    }
}

fn visit_cell(
    char_grid: &Vec<Vec<char>>, 
    distance_grid: &mut Vec<Vec<u32>>, 
    step:u32, 
    curr_position:(usize, usize)) {
    
    if step < distance_grid[curr_position.0][curr_position.1] {
        distance_grid[curr_position.0][curr_position.1] = step;
        for direction in DIRECTIONS {
            if let Some(new_position) = move_to_cell(curr_position, direction, char_grid) {
                if char::from_u32(char_grid[curr_position.0][curr_position.1] as u32 - 1).unwrap() <= char_grid[new_position.0][new_position.1] {
                    visit_cell(
                        char_grid, 
                        distance_grid, 
                        step+1, 
                        new_position,
                    )
                }
            }
        }
    }
}

fn shortest_path(input: &str) -> (u32, u32) {
    let (char_grid, 
        start_position, 
        end_position,
        minimum_height_cells) = parse_input(input);

    let mut distance_grid: Vec<Vec<u32>> = vec![vec![u32::MAX;char_grid[0].len()]; char_grid.len()];
    
    visit_cell(
        &char_grid, 
        &mut distance_grid, 
        0, 
        end_position);

    for line in &distance_grid {
        for dist in line {
            print!("{dist} ");
        }
        println!("");
    }
    
    let mut min = u32::MAX;

    for (x, y) in minimum_height_cells {
        if distance_grid[x][y] < min {
            min = distance_grid[x][y];
        }
    }

    (distance_grid[start_position.0][start_position.1], min)
}

fn main() {
    println!("Starting execution!\n");
    let start = Instant::now();

    let input = "./input.txt";
    
    let (shortest_from_b, shortest_path) = shortest_path(input);

    println!("The shortest path from the start to the end is {}", shortest_from_b);
    println!("The shortest path to the end is {}", shortest_path);
    

    let duration = start.elapsed();
    println!("\nTime elapsed in expensive_function() is: {:?}", duration);
}
