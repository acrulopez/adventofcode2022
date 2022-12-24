use std::collections::HashSet;
use std::time::Instant;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const GRID_DIMENSION:usize = 22;

#[derive(Clone)]

enum LavaDroplet {
    Lava,
    Water,
    Air
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_input(input: &str) -> Vec<(i32, i32, i32)> {
    let lines = read_lines(input)
        .unwrap_or_else(|_| panic!("File '{input}' should be accesible"));

    let mut cubes_coordinates = vec![];
    for line in lines {
        if let Ok(line) = line {
            let number_it = line.split(',');
            let mut coordinates = number_it.map(|x| x.parse::<i32>().unwrap());
            cubes_coordinates.push((coordinates.next().unwrap(), coordinates.next().unwrap(), coordinates.next().unwrap()));
        }
    }

    cubes_coordinates
}

fn solution_v1(input: &str) -> usize {

    let cubes_coordinates = parse_input(input);
    let mut lava_cubes = HashSet::new();
    let mut total_surface_area = 0;

    for coordinates in cubes_coordinates {
        total_surface_area += 6;
        lava_cubes.insert(coordinates);

        for (x,y,z) in [(1,0,0), (0,1,0), (0,0,1), (-1,0,0), (0,-1,0), (0,0,-1)] {
            if lava_cubes.contains(&(coordinates.0+x, coordinates.1+y, coordinates.2+z)) {
                total_surface_area -= 2;  
            }
        }
    }

    total_surface_area
}

fn parse_input_v2(input: &str) -> Vec<Vec<Vec<LavaDroplet>>> {
    let lines = read_lines(input)
        .unwrap_or_else(|_| panic!("File '{input}' should be accesible"));

    let mut grid: Vec<Vec<Vec<LavaDroplet>>> = vec![vec![vec![LavaDroplet::Air; GRID_DIMENSION]; GRID_DIMENSION]; GRID_DIMENSION];

    for line in lines {
        if let Ok(line) = line {
            let number_it = line.split(',');
            let mut coordinates = number_it.map(|x| x.parse::<usize>().unwrap());
            grid[coordinates.next().unwrap()][coordinates.next().unwrap()][coordinates.next().unwrap()] = LavaDroplet::Lava;
        }
    }

    grid
}

fn get_water_changes(position: (usize, usize, usize)) -> Vec<(isize, isize, isize)> {
    let mut water_changes = vec![];
    if position.0 > 0 {water_changes.push((-1,0,0))}
    if position.0 < GRID_DIMENSION-1 {water_changes.push((1,0,0))}

    if position.1 > 0 {water_changes.push((0,-1,0))}
    if position.1 < GRID_DIMENSION-1 {water_changes.push((0,1,0))}

    if position.2 > 0 {water_changes.push((0,0,-1))}
    if position.2 < GRID_DIMENSION-1 {water_changes.push((0,0,1))}

    water_changes
}

fn fill(grid: &mut Vec<Vec<Vec<LavaDroplet>>>, position: (usize, usize, usize), lava_encountered: u32) -> u32 {
    
    match grid[position.0][position.1][position.2] {
        LavaDroplet::Air => {
            grid[position.0][position.1][position.2] = LavaDroplet::Water;
            let mut sum = 0;
            for (x,y,z) in get_water_changes(position) {
                sum += fill(grid, (((position.0 as isize +x) as usize), ((position.1 as isize +y) as usize), ((position.2 as isize +z) as usize)), lava_encountered);
            }
            return sum;
        }
        LavaDroplet::Lava => {return 1;}
        LavaDroplet::Water => {return 0;}
    }
}

fn solution_v2(input: &str) -> u32 {

    let mut grid = parse_input_v2(input);

    // Flood fill algorithm
    let mut sum = 0;
    for x in 0..GRID_DIMENSION {
        for y in 0..GRID_DIMENSION {
            sum += fill(&mut grid, (x,y,0), 0);
            sum += fill(&mut grid, (x,0,y), 0);
            sum += fill(&mut grid, (0,x,y), 0);
            sum += fill(&mut grid, (x,y,GRID_DIMENSION-1), 0);
            sum += fill(&mut grid, (x,GRID_DIMENSION-1,y), 0);
            sum += fill(&mut grid, (GRID_DIMENSION-1,x,y), 0);
        }    
    }

    sum
}


fn solution(input: &str) -> (usize, u32) {

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
