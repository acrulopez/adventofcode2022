use std::time::Instant;
use std::{env, vec};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const BASE:i128 = 5;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_input(input: &str) -> Vec<i128> {
    let lines = read_lines(input)
        .unwrap_or_else(|_| panic!("File '{input}' should be accesible"));


    let numbers = lines
        .map(|x| snafu_to_decimal(x.unwrap()))
        .collect();


    numbers
}

fn char_to_number(ch: char) -> i128 {
    match ch {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '-' => -1,
        '=' => -2,
        _ => panic!("should be one of those options")
    }
}

fn number_to_char(n: i128) -> (char, bool) {
    let mut carry = false;
    let ch: char;
    match n {
        0 => ch = '0',
        1 => ch = '1',
        2 => ch = '2',
        3 => {ch = '='; carry = true;},
        4 => {ch = '-'; carry = true;},
        5 => {ch = '0'; carry = true;},
        _ => panic!("should be one of those options")
    }

    (ch, carry)
}

fn snafu_to_decimal(n_snafu: String) -> i128 {

    return 
    n_snafu.chars().rev().enumerate()
    .map(|(x, y)| BASE.pow(x as u32) * char_to_number(y))
    .sum::<i128>();

}

fn decimal_to_snafu(mut number: i128) -> String {

    let mut str = String::new();

    let mut current_power = 0;
    loop {
        if number / BASE.pow(current_power) < 5 {
            break;
        } 
        current_power += 1;
    }
    
    let mut divs = vec![];
    let mut pow;
    loop {
        pow = BASE.pow(current_power);

        divs.push(number / pow);
        number %= pow;

        if current_power == 0 {break;}
        current_power -= 1;
    }

    let mut carry = false;
    let mut ch;
    for mut div in divs.into_iter().rev() {
        if carry {div += 1;}
        (ch, carry) = number_to_char(div);
        str.push(ch);
    }

    if carry {str.push('1')}

    str = str.chars().rev().collect::<String>();

    str
}

fn solution(input: &str) -> String {
    let numbers = parse_input(input);

    let sum = numbers.iter().sum::<i128>();

    let snafu = decimal_to_snafu(sum);

    snafu
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
    
    let solution_v1 = solution(input);
    println!("V1 solution is {}", solution_v1);

    let duration = start.elapsed();
    println!("\nTime elapsed is: {:?}", duration);

}
