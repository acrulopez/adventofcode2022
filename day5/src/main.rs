use std::time::Instant;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::io::Read;
use regex::Regex;

fn read_lines<P>(filename: P) -> io::Result<io::BufReader<File>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file))
}

fn fill_stack_by_line(stacks:&mut Vec<Vec<char>>, line:&str) {
    let mut line = line.chars();

    // Remove first char
    line.next();

    // Take the first one
    let first_stack_char = line.next().unwrap();
    // Leave if there is a number
    if first_stack_char.is_digit(10) {return ()}
    // Add to the stack if it's not a whitespace char
    if !first_stack_char.is_whitespace() {stacks[0].push(first_stack_char)}
    
    // Iterate over the rest
    let mut next_char;
    for i in 1..stacks.len() {
        next_char = line.nth(3).unwrap();
        if !next_char.is_whitespace() {stacks[i].push(next_char)}
    }
}

fn compute_stacks(input: &str) -> String {
    let mut buff_reader = read_lines(input)
        .unwrap_or_else(|_| panic!("File '{input}' should be accesible"));

    // Take the iterator
    let mut lines = buff_reader.by_ref().lines();

    // Take the first line to initialize the stacks
    let first_line = lines.next().unwrap().unwrap();
    let n_stacks = (first_line.len() + 1) / 4;
    let mut stacks: Vec<Vec<char>> = vec![vec![]; n_stacks];

    // Fill the stack
    fill_stack_by_line(&mut stacks, &first_line);
    while let Some(Ok(line)) = lines.next() {
        if !line.is_empty() {
            fill_stack_by_line(&mut stacks, &line);
        }
        else {break} // Stop when there is a line break
    }

    // Reverse the stacks because they were filled on reverse order
    for stack in &mut stacks {
        stack.reverse();
    }

    // Move the crates
    let re = Regex::new(r"\d+").unwrap();
    for line in lines {
        let nums:Vec<usize> = re.find_iter(&line.as_ref().unwrap())
            .filter_map(|digits| digits.as_str().parse::<usize>().ok()).collect();
        
        for _i in 0..nums[0] {
            let popped_element = stacks[nums[1]-1].pop().unwrap();
            stacks[nums[2]-1].push(popped_element);
        }

    }
    
    let mut top_elements = String::new();
    for stack in stacks {
        top_elements.push(*stack.last().unwrap());
    }
    top_elements
}

fn compute_stacks_v2(input: &str) -> String {
    let mut buff_reader = read_lines(input)
        .unwrap_or_else(|_| panic!("File '{input}' should be accesible"));

    // Take the iterator
    let mut lines = buff_reader.by_ref().lines();

    // Take the first line to initialize the stacks
    let first_line = lines.next().unwrap().unwrap();
    let n_stacks = (first_line.len() + 1) / 4;
    let mut stacks: Vec<Vec<char>> = vec![vec![]; n_stacks];

    // Fill the stack
    fill_stack_by_line(&mut stacks, &first_line);
    while let Some(Ok(line)) = lines.next() {
        if !line.is_empty() {
            fill_stack_by_line(&mut stacks, &line);
        }
        else {break} // Stop when there is a line break
    }

    // Reverse the stacks because they were filled on reverse order
    for stack in &mut stacks {
        stack.reverse();
    }

    // Move the crates
    let re = Regex::new(r"\d+").unwrap();
    for line in lines {
        let nums:Vec<usize> = re.find_iter(&line.as_ref().unwrap())
            .filter_map(|digits| digits.as_str().parse::<usize>().ok()).collect();
        
        
        let current_len = stacks[nums[1]-1].len();
        let popped_elements:Vec<char> = stacks[nums[1]-1].drain(current_len-nums[0]..).collect();
        for element in popped_elements {stacks[nums[2]-1].push(element)}

    }
    
    let mut top_elements = String::new();
    for stack in stacks {
        top_elements.push(*stack.last().unwrap());
    }
    top_elements
}

fn main() {
    println!("Starting execution!\n");
    let start = Instant::now();

    let input = "./input.txt";
    
    let top_elements = compute_stacks(input);
    let top_elements_v2 = compute_stacks_v2(input);

    println!("The top elements are: {}", top_elements);
    println!("The top elements on v2 are: {}", top_elements_v2);
    

    let duration = start.elapsed();
    println!("\nTime elapsed in expensive_function() is: {:?}", duration);
}
