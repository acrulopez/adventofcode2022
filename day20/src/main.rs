use std::time::Instant;
use std::env;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const DECRYPTION_KEY: i64 = 811589153;
const N_MIXINGS:usize = 10;

#[derive(Debug)]
struct Value {
    value: i64,
    idx: usize
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_input(input: &str, factor:i64) -> Vec<Value> {
    let lines = read_lines(input)
        .unwrap_or_else(|_| panic!("File '{input}' should be accesible"));

    
    let number_list: Vec<Value> = lines
        .enumerate()
        .map(|(idx, value)| Value{ value: value.unwrap().parse::<i64>().unwrap()*factor, idx})
        .collect();

    number_list
}

fn mix_number_list(number_list: &mut Vec<Value>, times: usize) {
    let list_len = number_list.len();

    let mut current_pos;
    let mut new_position;
    let mut number;
    for _ in 0..times {
        for idx in 0..list_len {
            current_pos = number_list.iter().position(|x| x.idx == idx).unwrap();
            number = number_list[current_pos].value;
            new_position = (current_pos as i64 + number).rem_euclid((list_len-1) as i64) as usize;
            number_list.remove(current_pos);
            number_list.insert(new_position, Value { value: number, idx: idx});
        }
    }
}

pub fn solution_v1(input: &str) -> i64 {
    let mut number_list = parse_input(input, 1);
    let list_len = number_list.len();

    mix_number_list(&mut number_list, 1);

    let zero_index = number_list.iter().position(|x| x.value == 0).unwrap();

    let mut sum = 0;
    for i in [1000,2000,3000] {
        sum += &number_list[(zero_index + i) % list_len].value
    }
    sum
}

fn solution_v2(input: &str) -> i64 {
    let mut number_list = parse_input(input, DECRYPTION_KEY);
    let list_len = number_list.len();

    mix_number_list(&mut number_list, N_MIXINGS);

    let zero_index = number_list.iter().position(|x| x.value == 0).unwrap();

    let mut sum = 0;
    for i in [1000,2000,3000] {
        sum += &number_list[(zero_index + i) % list_len].value
    }
    sum
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
    
    let solution_v1 = solution_v1(input);
    println!("V1 solution is {}", solution_v1);

    let duration = start.elapsed();
    println!("\nTime elapsed is: {:?}", duration);
    
    // --------------------------V2--------------------------------

    println!("Starting execution version 2!\n");
    let start = Instant::now();

    let solution_v2 = solution_v2(input);
    println!("V2 solution is {}", solution_v2);
    

    let duration = start.elapsed();
    println!("\nTime elapsed is: {:?}", duration);
}
