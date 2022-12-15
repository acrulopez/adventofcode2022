use core::panic;
use std::time::Instant;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;
use regex::Regex;
use std::collections::VecDeque;

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u128>,
    operations:Vec<String>,
    divisible_by: u128,
    throw_to_if_true: u128,
    throw_to_if_false: u128,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn monkey_operation(operations:&Vec<String>, old:u128) -> u128 {
    let var1 = match &operations[0][..] {
        "old" => old,
        number => number.parse::<u128>().unwrap(),
    };

    let var2 = match &operations[2][..] {
        "old" => old,
        number => number.parse::<u128>().unwrap(),
    };

    return match &operations[1][..] {
        "+" => var1 + var2,
        "*" => var1 * var2,
        _ => panic!("Should be either '*' or '+'")
    };
}

fn fill_next_monkey_input(monkeys: &mut Vec<Monkey>, lines: &mut Lines<BufReader<File>>) {
    // Get equiped items
    let number_re = Regex::new(r"\d+").unwrap();
    let items:VecDeque<u128> = VecDeque::from_iter(
        number_re.find_iter(&lines.next().unwrap().unwrap())
            .filter_map(|digits| digits.as_str().parse::<u128>().ok())
    );
    
    // Create inspection operation
    let operations = lines.next().unwrap().unwrap();
    let operations: Vec<String> = operations.trim().split("=")
        .collect::<Vec<&str>>()[1]
        .trim().split(" ").map(|s| s.to_string()).collect::<Vec<String>>();

    // Get test from monkey
    let divisible_by = number_re.find(&lines.next().unwrap().unwrap()).
        unwrap().as_str().parse::<u128>().unwrap();

    // Get monkey int to throw if test is true
    let throw_to_if_true = number_re.find(&lines.next().unwrap().unwrap()).
        unwrap().as_str().parse::<u128>().unwrap();

    // Get monkey int to throw if test is false
    let throw_to_if_false = number_re.find(&lines.next().unwrap().unwrap()).
        unwrap().as_str().parse::<u128>().unwrap();

    // Discard empty line
    lines.next();

    // Push money to vector
    monkeys.push(
        Monkey {
            items,
            operations,
            divisible_by,
            throw_to_if_true,
            throw_to_if_false,
        }
    )

    
}

fn parse_input(input: &str) -> Vec<Monkey>  {
    let mut lines = read_lines(input)
        .unwrap_or_else(|_| panic!("File '{input}' should be accesible"));

    let mut monkeys = vec![];

    while let Some(_) = lines.next() {
        fill_next_monkey_input(&mut monkeys, &mut lines);
    }

    monkeys
}


fn calculate_monkey_business(input: &str) -> usize  {
    let mut monkeys = parse_input(input);

    let mut inspected_items = vec![0; monkeys.len()];
    let mut thrown_items = vec![];

    for _ in 0..20 {
        for n_monk in 0..monkeys.len() {
            let monkey: &mut Monkey = &mut (&mut monkeys)[n_monk];
            inspected_items[n_monk] += monkey.items.len();

            // Calculate the thrown items
            while let Some(item) = monkey.items.pop_front() {
                let op_result = monkey_operation(&monkey.operations,item) / 3;

                if op_result%monkey.divisible_by == 0 {
                    thrown_items.push((monkey.throw_to_if_true, op_result));
                }
                else {
                    thrown_items.push((monkey.throw_to_if_false, op_result));
                }
            }
            // Throw the items
            for (to, worry_level) in &thrown_items {
                monkeys[*to as usize].items.push_back(*worry_level);
            }
            thrown_items.clear();
        }
    }

    let (mut max1, mut max2) = (0, 0);
    // Iterate through the vector and find the two maximum values
    for number in inspected_items {
        if number > max1 {
            max2 = max1;
            max1 = number;
        } else if number > max2 {
            max2 = number;
        }
    }

    max1*max2
}


fn calculate_monkey_business_v2(input: &str) -> usize  {
    let mut monkeys = parse_input(input);
    let module = (&monkeys).iter().map(|x| x.divisible_by).reduce(|x, y| x*y).unwrap();

    let mut inspected_items = vec![0; monkeys.len()];
    let mut thrown_items = vec![];

    for _ in 0..10000 {
        for n_monk in 0..monkeys.len() {
            let monkey: &mut Monkey = &mut (&mut monkeys)[n_monk];
            inspected_items[n_monk] += monkey.items.len();

            // Calculate the thrown items
            while let Some(item) = monkey.items.pop_front() {
                let op_result = monkey_operation(&monkey.operations,item) % module;

                if op_result%monkey.divisible_by == 0 {
                    thrown_items.push((monkey.throw_to_if_true, op_result));
                }
                else {
                    thrown_items.push((monkey.throw_to_if_false, op_result));
                }
            }
            // Throw the items
            for (to, worry_level) in &thrown_items {
                monkeys[*to as usize].items.push_back(*worry_level);
            }
            thrown_items.clear();
        }
    }

    let (mut max1, mut max2) = (0, 0);
    // Iterate through the vector and find the two maximum values
    for number in inspected_items {
        if number > max1 {
            max2 = max1;
            max1 = number;
        } else if number > max2 {
            max2 = number;
        }
    }

    max1*max2
}
fn main() {
    println!("Starting execution!\n");
    let start = Instant::now();

    let input = "./input.txt";
    
    let monkey_bussiness = calculate_monkey_business(input);
    let monkey_bussiness_v2 = calculate_monkey_business_v2(input);

    println!("The monkey business is {monkey_bussiness}");
    println!("The monkey business for v2 is {monkey_bussiness_v2}");

    let duration = start.elapsed();
    println!("\nTime elapsed is: {:?}", duration);
}
