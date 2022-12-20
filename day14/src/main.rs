use std::time::Instant;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

const FALLING_FROM: usize = 500;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_grid(input: &str) -> (Vec<Vec<bool>>, u32){
    let lines = read_lines(input)
        .unwrap_or_else(|_| panic!("File '{input}' should be accesible"));

    let mut stone_ranges = vec![];
    let (mut max_x, mut max_y, mut min_x) = (u32::MIN, u32::MIN, u32::MAX);
    let mut last_vec;

    let number_re = Regex::new(r"\d+,\d+").unwrap();
    for line in lines {
        if let Ok(line) = line {

            stone_ranges.push(vec![]);
            last_vec = stone_ranges.last_mut().unwrap();

            let coordinates = number_re
            .find_iter(&line).filter_map(|number| Some(number.as_str()))
            .collect::<Vec<&str>>();
            for point in coordinates {
                let mut it = point.split(',').map(|coordinate| coordinate.parse::<u32>());
                let point = (it.next().unwrap().unwrap(), it.next().unwrap().unwrap());
                if point.0 > max_x {max_x = point.0}
                if point.0 < min_x {min_x = point.0}
                if point.1 > max_y {max_y = point.1}
                last_vec.push(point);
            }
        }
    }
    min_x -= 1;
    max_x += 1;
    let diff_x = max_x - min_x;

    let mut grid = vec![];

    for _ in 0..diff_x+1 {grid.push(vec![false; (max_y+1) as usize])}

    for stone_range in stone_ranges {
        let mut last_point = stone_range[0];
        for point in stone_range {
            if point.0 == last_point.0 {
                if last_point.1 < point.1 {
                    for stone_coordinate in last_point.1..point.1+1 {
                        grid[(point.0-min_x) as usize][(stone_coordinate) as usize] = true;
                    }
                }
                else {
                    for stone_coordinate in point.1..last_point.1+1 {
                        grid[(point.0-min_x) as usize][(stone_coordinate) as usize] = true;
                    }
                }
            }
            else {
                if last_point.0 < point.0 {
                    for stone_coordinate in last_point.0..point.0+1 {
                        grid[(stone_coordinate-min_x) as usize][(point.1) as usize] = true;
                    }
                }
                else {
                    for stone_coordinate in point.0..last_point.0+1 {
                        grid[(stone_coordinate-min_x) as usize][(point.1) as usize] = true;
                    }
                }
            }
            last_point = point;
        }
    }
    
    (grid, min_x)

}

fn solution(mut grid: Vec<Vec<bool>>, falling_from: (usize, usize)) -> (Vec<Vec<bool>>, u32) {

    // let (mut grid, min_x) = get_grid(input);
    let size_x = grid.len();
    let size_y = grid[0].len();

    let mut n_sands = 0;
    let mut sand_pos = falling_from;
    let mut historical_pos = vec![sand_pos];

    'outer: loop {
        sand_pos = historical_pos.pop().unwrap();

        'inner: loop {
            historical_pos.push(sand_pos);
            if sand_pos.1 == size_y-1 {
                break 'outer;
            }
            if !grid[sand_pos.0][sand_pos.1+1] {
                sand_pos.1 += 1;
            }
            else if sand_pos.0 != 0 && !grid[sand_pos.0-1][sand_pos.1+1] {
                sand_pos.1 += 1;
                sand_pos.0 -= 1;
            }
            else if sand_pos.0 != size_x-1 && !grid[sand_pos.0+1][sand_pos.1+1] {
                sand_pos.1 += 1;
                sand_pos.0 += 1;
            }
            else {
                // The sand found a spot
                grid[sand_pos.0][sand_pos.1] = true;
                n_sands += 1;
                if sand_pos == falling_from {break 'outer; }

                historical_pos.pop();
                break 'inner;
            }
        }
    }

    (grid, n_sands)
}

fn solution_v1(input: &str) -> u32 {
    let (grid, min_x) = get_grid(input);
    let (_, n_sands) = solution(grid, (FALLING_FROM-min_x as usize, 0));
    n_sands
}

fn solution_v2(input: &str) -> u32 {
    let (mut grid, min_x) = get_grid(input);
    for row in grid.iter_mut() {
        row.push(false);
        row.push(true);
    }
    let (grid, mut n_sands) = solution(grid, (FALLING_FROM-min_x as usize, 0));

    let len_y = grid[0].len();
    let index_left = grid[0].iter().position(|&r| r == true).unwrap();
    let index_right = grid.last().unwrap().iter().position(|&r| r == true).unwrap();

    n_sands += (((len_y-index_left-2) * (len_y-index_left-1)) / 2) as u32;
    n_sands += (((len_y-index_right-2) * (len_y-index_right-1)) / 2) as u32;

    n_sands
}

fn main() {
    println!("Starting execution!\n");
    let start = Instant::now();

    let input = "./input.txt";
    
    let sand_units_v1 = solution_v1(input);
    let sand_units_v2 = solution_v2(input);

    println!("The number of sand units is {}", sand_units_v1);
    println!("The number of sand units is {}", sand_units_v2);
    

    let duration = start.elapsed();
    println!("\nTime elapsed in expensive_function() is: {:?}", duration);
}
