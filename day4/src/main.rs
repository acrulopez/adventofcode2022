use std::time::Instant;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn elf_contains(elf_assignments: &[[u32; 2]; 2]) -> bool {
    if (elf_assignments[0][0] <= elf_assignments[1][0] 
            &&
        elf_assignments[0][1] >= elf_assignments[1][1]) 
        ||  
        (elf_assignments[0][0] >= elf_assignments[1][0] 
            &&
        elf_assignments[0][1] <= elf_assignments[1][1]) {
            true
        }
    else {
        false
    }
}

fn elf_overlaps(elf_assignments: &[[u32; 2]; 2]) -> bool {
    if elf_contains(elf_assignments) || 
        (elf_assignments[0][1] >= elf_assignments[1][0] &&
            elf_assignments[0][0] <= elf_assignments[1][0])
        || 
        (elf_assignments[1][1] >= elf_assignments[0][0] && 
            elf_assignments[1][0] <= elf_assignments[0][0]){
            true
        }
    else {
        false
    }
}

fn parse_pairs(input: &str) -> Vec<[[u32; 2]; 2]> {
    let lines = read_lines(input)
        .unwrap_or_else(|_| panic!("File '{input}' should be accesible"));

    let mut vec = Vec::new();
    let mut elf_idx;
    let mut assign_idx;
    let mut num_str;

    for line in lines {
        if let Ok(line) = line {
            let mut elf_assignments = [[0,0], [0,0]];
            elf_idx = 0;
            assign_idx = 0;
            num_str = String::new();
            for character in line.chars() {
                match character {
                    '-' => {
                        elf_assignments[elf_idx][assign_idx] = num_str.parse()
                            .expect("Should be a number");
                        assign_idx = 1;
                        num_str.clear()
                    }
                    ',' => {
                        elf_assignments[elf_idx][assign_idx] = num_str.parse()
                            .expect("Should be a number");
                        elf_idx = 1;
                        assign_idx = 0;
                        num_str.clear();
                    }
                    number => num_str.push(number),
                }
            }
            // Add the end assignment of the last elf
            elf_assignments[elf_idx][assign_idx] = num_str.parse()
                .expect("Should be a number");
            vec.push(elf_assignments);
        }
    }
    vec
}

fn get_containing_overlapping_pairs(input: &str) -> (u32, u32) {

    let elf_assignments = parse_pairs(input);

    let mut containing_pairs = 0;
    let mut overlapping_pairs = 0;
    for assignments in elf_assignments {
        if elf_contains(&assignments) {
            containing_pairs += 1;
        }
        if elf_overlaps(&assignments) {
            overlapping_pairs += 1;
        }
    }
    (containing_pairs, overlapping_pairs)
}

fn main() {
    println!("Starting execution!\n");
    let start = Instant::now();

    let input = "./input.txt";
    
    let (containing_pairs, overlapping_pairs) = get_containing_overlapping_pairs(input);

    println!("The number of pairs containing each other : {containing_pairs}");
    println!("The number of pairs overlapping each other : {overlapping_pairs}");

    let duration = start.elapsed();
    println!("\nTime elapsed in expensive_function() is: {:?}", duration);
}
