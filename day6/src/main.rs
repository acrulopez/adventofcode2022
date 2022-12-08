use std::time::Instant;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_marker_idx(input: &str, distinct_chars:isize) -> u32 {
    let lines = read_lines(input)
        .unwrap_or_else(|_| panic!("File '{input}' should be accesible"));

 
    let mut char_to_idx: [isize; 26] = [-1; 26];
    let mut current_idx:isize = 0;
    let mut skip_chars = distinct_chars - 1;
    let mut current_char;
    let mut distance;

    for line in lines {
        if let Ok(line) = line {
            for c in line.bytes() {
                current_char = (c-97) as usize;
                distance = current_idx - char_to_idx[current_char];
                if skip_chars == 0 {
                    if current_idx - char_to_idx[current_char] > (distinct_chars-1) {
                        return current_idx as u32
                    }
                    else {
                        if skip_chars < distinct_chars - distance {skip_chars = distinct_chars - distance}
                    }
                }
                else {
                    skip_chars -= 1;
                    if distance <= (distinct_chars-1) {
                        if skip_chars < distinct_chars - distance {skip_chars = distinct_chars - distance}
                    }
                }
                char_to_idx[current_char] = current_idx;

                current_idx += 1;
            }
        }      
    }
    0
}

fn main() {
    println!("Starting execution!\n");
    let start = Instant::now();

    let input = "./input.txt";
    
    let marker4 = get_marker_idx(input, 4);
    let marker14 = get_marker_idx(input, 14);

    println!("The marker is in position: {marker4} for 4 chars");
    println!("The marker is in position: {marker14} for 14 chars");

    let duration = start.elapsed();
    println!("\nTime elapsed in expensive_function() is: {:?}", duration);
}
