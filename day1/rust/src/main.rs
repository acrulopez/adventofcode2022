use std::time::Instant;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn top1_calories(input: &str) -> u32 {
    let lines = read_lines(input)
        .unwrap_or_else(|_| panic!("File '{input}'should be accesible"));

    let mut max_calories = 0;
    let mut sum_calories = 0;
    for line in lines {
        if let Ok(line) = line {
            let calories = line.parse::<u32>();
            if let Ok(calories) = calories {
                sum_calories += calories;
            }
            else {
                if max_calories < sum_calories {max_calories = sum_calories}
                sum_calories = 0;
            }
        }      
    }

    max_calories
}

fn top3_calories(input: &str) -> u32 {
    let lines = read_lines(input)
        .unwrap_or_else(|_| panic!("File '{input}'should be accesible"));

    let mut max_calories = [0,0,0];
    let mut sum_calories = 0;
    for line in lines {
        if let Ok(line) = line {
            let calories = line.parse::<u32>();
            if let Ok(calories) = calories {
                sum_calories += calories;
            }
            else {
                if sum_calories < max_calories[2] {}
                else if sum_calories > max_calories[0] {
                    let aux = max_calories[0];
                    let aux1 = max_calories[1];
                    max_calories[0] = sum_calories;
                    max_calories[1] = aux;
                    max_calories[2] = aux1;
                }
                else if sum_calories > max_calories[1] {
                    let aux1 = max_calories[1];
                    max_calories[1] = sum_calories;
                    max_calories[2] = aux1;
                }
                else {max_calories[2] = sum_calories}
                sum_calories = 0;
            }
        }
    }

    max_calories[0] + max_calories[1] + max_calories[2]
}


fn main() {
    println!("Starting execution!\n");
    let start = Instant::now();

    let input = "../input.txt";
    
    let max_calories = top1_calories(input);
    let max3_calories = top3_calories(input);

    println!("The number of calories of the Elf carrying the most is {max_calories}\n");
    println!("The number of calories of the three Elf carrying the most is {max3_calories}");

    let duration = start.elapsed();
    println!("\nTime elapsed in expensive_function() is: {:?}", duration);
}


