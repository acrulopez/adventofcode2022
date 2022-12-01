use std::time::Instant;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn main() {
    println!("Starting execution!\n");
    let start = Instant::now();

    let input = "../input.txt";
    let lines = read_lines(input)
        .unwrap_or_else(|_| panic!("File '{input}'should be accesible"));

    let mut max_calories = 0;
    let mut sum_calories = 0;
    for line in lines {
        if let Ok(line) = line {
            let line = line.parse::<u32>();
            if let Ok(line) = line {
                sum_calories += line;
            }
            else {
                if max_calories < sum_calories {max_calories = sum_calories}
                sum_calories = 0;
            }
        }      
    }

    println!("The number of calories of the Elf carrying the most is {max_calories}");

    let duration = start.elapsed();
    println!("\nTime elapsed in expensive_function() is: {:?}", duration);
}


