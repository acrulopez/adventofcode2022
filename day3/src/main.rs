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

fn get_priorities(input: &str) -> u32 {
    let lines = read_lines(input)
        .unwrap_or_else(|_| panic!("File '{input}' should be accesible"));

    let mut priorities = 0;

    for line in lines {
        if let Ok(line) = line {
            let mut seen_items = HashSet::new();
            let chars_vec: &[u8] = line.as_bytes();
            let num_items = chars_vec.len();

            for i in 0..num_items/2 {
                seen_items.insert(chars_vec[i]);
            }

            for i in num_items/2..num_items {
                if seen_items.contains(&chars_vec[i]) {
                    // Lowercase characters start on 97 (in decimal).
                    if chars_vec[i] > 96 {priorities += chars_vec[i] as u32 - 96}
                    // Lowercase characters start on 65 (in decimal). We substract 38 to
                    // convert 65 into 27, the value of the first capital letter A.
                    else {priorities += chars_vec[i] as u32 - 38}
                    // Stop the loop to avoid counting more than once one line
                    break;
                }
            }
        }      
    }
    priorities
}

fn get_group_priorities(input: &str) -> u32 {
    let lines = read_lines(input)
        .unwrap_or_else(|_| panic!("File '{input}' should be accesible"));

    let mut priorities = 0;
    let mut number_in_group = 0;

    let mut seen_items = HashSet::new();
    let mut seen_items2 = HashSet::new();

    for line in lines {
        if let Ok(line) = line {
            let chars_vec: &[u8] = line.as_bytes();
            let num_items = chars_vec.len();

            match number_in_group {
                0 => {
                    for i in 0..num_items {
                        seen_items.insert(chars_vec[i]);
                    }
                    number_in_group = (number_in_group + 1) % 3;
                }
                1 => {
                    for i in 0..num_items {
                        seen_items2.insert(chars_vec[i]);
                    }
                    number_in_group = (number_in_group + 1) % 3;
                }
                2 => {
                    for i in 0..num_items {
                        if seen_items.contains(&chars_vec[i]) & seen_items2.contains(&chars_vec[i]){
                            // Lowercase characters start on 97 (in decimal).
                            if chars_vec[i] > 96 {priorities += chars_vec[i] as u32 - 96}
                            // Lowercase characters start on 65 (in decimal). We substract 38 to
                            // convert 65 into 27, the value of the first capital letter A.
                            else {priorities += chars_vec[i] as u32 - 38}
                            // Stop the loop to avoid counting more than once one line
                            break;
                        }
                    }
                    number_in_group = (number_in_group + 1) % 3;
                    seen_items.clear();
                    seen_items2.clear();
                }
                _ => panic!("Number in group can only be [0,1,2]"),
            }
        }      
    }
    priorities
}

fn main() {
    println!("Starting execution!\n");
    let start = Instant::now();

    let input = "./input.txt";
    
    let priorities = get_priorities(input);
    let group_priorities = get_group_priorities(input);

    println!("The sum of priorities is: {priorities}");
    println!("The sum of the group priorities is: {group_priorities}");

    let duration = start.elapsed();
    println!("\nTime elapsed in expensive_function() is: {:?}", duration);
}
