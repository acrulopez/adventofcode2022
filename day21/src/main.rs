use std::time::Instant;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{HashMap};
use regex::Regex;

#[derive(Clone)]
enum Monkey {
    MonkeyOperation(MonkeyOperation),
    MonkeyNumber(i64)
}

#[derive(Clone)]
struct MonkeyOperation {
    m1: String,
    m2: String,
    operation: char,
}

#[derive(Debug)]
enum Operation {
    Add,
    SubstractX,
    XSubstract,
    Multiply,
    IsDivided,
    DivideBy
}

#[derive(Debug)]
enum OperationResult {
    Number(i64),
    Operations(Vec<(i64, Operation)>)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_input(input: &str) -> HashMap<String, Monkey> {
    let lines = read_lines(input)
        .unwrap_or_else(|_| panic!("File '{input}' should be accesible"));

    
    let mut monkey_map = HashMap::new();

    let re = Regex::new(r"[a-zA-Z0-9+*\-/]+").unwrap();
    let mut monkey_action: Vec<String>;
    for line in lines {
        if let Ok(line) = line {
            monkey_action = re
            .find_iter(&line).filter_map(|number| Some(number.as_str().to_owned()))
            .collect();
            match monkey_action.len() {
                2 => {
                    monkey_map.insert(monkey_action[0].to_string(), Monkey::MonkeyNumber(monkey_action[1].parse::<i64>().unwrap()));
                }
                4 => {
                    monkey_map.insert(monkey_action[0].to_string(), Monkey::MonkeyOperation(MonkeyOperation {
                        m1: monkey_action[1].to_string(),
                        m2: monkey_action[3].to_string(),
                        operation: monkey_action[2].chars().next().unwrap(),
                    }));
                }
                _ => panic!("len should be 2 or 4")
            }
        }
    }

    monkey_map
}

fn calculate_result(monkey_map: &HashMap<String, Monkey>, root: &str) -> i64 {
    let monkey = monkey_map.get(root).unwrap().clone();
    match monkey {
        Monkey::MonkeyNumber(number) => return number,
        Monkey::MonkeyOperation(monkey_operation) => {

            let m1 = calculate_result(monkey_map, &monkey_operation.m1);
            let m2 = calculate_result(monkey_map, &monkey_operation.m2);
            
            let result = match monkey_operation.operation {
                '+' => m1 + m2,
                '-' => m1 - m2,
                '/' => m1 / m2,
                '*' => m1 * m2,
                _ => panic!("should be * - / +")
            };
            return result;
        }
    }
}

fn solution_v1(input: &str) -> i64 {
    let mut monkey_map = parse_input(input);
    
    return calculate_result(&mut monkey_map, "root");
}

fn calculate_result_v2(monkey_map: &HashMap<String, Monkey>, root: &str) -> OperationResult {
    if root == "humn" {
        return OperationResult::Operations(vec![]);
    }
    let monkey = monkey_map.get(root).unwrap().clone();
    match monkey {
        Monkey::MonkeyNumber(number) => return OperationResult::Number(number),
        Monkey::MonkeyOperation(monkey_operation) => {

            let mut monkey1 = calculate_result_v2(monkey_map, &monkey_operation.m1);
            let mut monkey2 = calculate_result_v2(monkey_map, &monkey_operation.m2);
            
            if let (OperationResult::Number(m1), OperationResult::Number(m2)) = (&monkey1, &monkey2) {
                let result = match monkey_operation.operation {
                    '+' => *m1 + *m2,
                    '-' => *m1 - *m2,
                    '/' => *m1 / *m2,
                    '*' => *m1 * *m2,
                    _ => panic!("should be * - / +")
                };
                return OperationResult::Number(result);
            }

            if let (OperationResult::Operations(m1), OperationResult::Number(m2)) = (&mut monkey1, &monkey2) {
                match monkey_operation.operation {
                    '+' => m1.push((*m2, Operation::Add)),
                    '-' => m1.push((*m2, Operation::SubstractX)),
                    '/' => m1.push((*m2, Operation::DivideBy)),
                    '*' => m1.push((*m2, Operation::Multiply)),
                    _ => panic!("should be * - / +")
                };
                return monkey1;
            }

            if let (OperationResult::Number(m1), OperationResult::Operations(m2)) = (&monkey1, &mut monkey2) {
                match monkey_operation.operation {
                    '+' => m2.push((*m1, Operation::Add)),
                    '-' => m2.push((*m1, Operation::XSubstract)),
                    '/' => m2.push((*m1, Operation::IsDivided)),
                    '*' => m2.push((*m1, Operation::Multiply)),
                    _ => panic!("should be * - / +")
                };
                return monkey2;
            }

            panic!("Should have enter in an if statement");
        }
    }
}

fn solution_v2(input: &str) -> i64 {
    let mut monkey_map = parse_input(input);
    
    let a = calculate_result_v2(&mut monkey_map, "root");
    match a {
        OperationResult::Number(number) => return number,
        OperationResult::Operations(mut operations) => {
            let mut equals_to = operations.pop().unwrap().0;
            while let Some((n,op)) = operations.pop() {
                match op {
                    Operation::Add => {equals_to -= n}
                    Operation::SubstractX => {equals_to += n}
                    Operation::XSubstract => {equals_to = n - equals_to}
                    Operation::Multiply => {equals_to /= n}
                    Operation::DivideBy => {equals_to *= n}
                    Operation::IsDivided => {equals_to = n / equals_to}
                }
            }
            return equals_to;
        }
    }
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
