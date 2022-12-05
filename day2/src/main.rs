use std::time::Instant;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// 0 for rock, 1 for paper, 2 for scissors. First index is my choice, second is their choice
const RPS_RESULTS: [[u32; 3]; 3] = [[3,0,6], [6,3,0],[0,6,3]];

// 1st index: 0 for rock, 1 for paper, 2 for scissors
// 2nd index: 0 for lose, 1 for tie, 2 for win
const WINNING_STRAT: [[u32; 3]; 3] = [[3,1,2], [1,2,3],[2,3,1]];

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_tournament_score(input: &str) -> u32 {
    let lines = read_lines(input)
        .unwrap_or_else(|_| panic!("File '{input}' should be accesible"));

    let mut score:u32 = 0;
    let mut my_choice:usize;
    for line in lines {
        if let Ok(line) = line {
            let chars_vec: &[u8] = line.as_bytes();
            match chars_vec[2] as char {
                'X' => {score += 1; my_choice=0;} 
                'Y' => {score += 2; my_choice=1;}
                'Z' => {score += 3; my_choice=2;}
                other_char => panic!("Found {other_char} on input. Possible values ['X', 'Y', 'Z']")
            }
            match chars_vec[0] as char {
                'A' => score += RPS_RESULTS[my_choice][0],
                'B' => score += RPS_RESULTS[my_choice][1],
                'C' => score += RPS_RESULTS[my_choice][2],
                other_char => panic!("Found {other_char} on input. Possible values ['A', 'B', 'C']")
            }
        }      
    }
    score
}

fn get_real_tournament_score(input: &str) -> u32 {
    let lines = read_lines(input)
        .unwrap_or_else(|_| panic!("File '{input}' should be accesible"));

    let mut score:u32 = 0;
    for line in lines {
        if let Ok(line) = line {
            let chars_vec: &[u8] = line.as_bytes();
            // Rock char 'X' is 88, so we convert it to 0, 1 for paper and 2 for scissors
            let round_outcome = (chars_vec[2] - 88) as usize; 
            let outcome_points = round_outcome as u32 * 3;
            match chars_vec[0] as char {
                'A' => score += WINNING_STRAT[0][round_outcome] + outcome_points,
                'B' => score += WINNING_STRAT[1][round_outcome] + outcome_points,
                'C' => score += WINNING_STRAT[2][round_outcome] + outcome_points,
                other_char => panic!("Found {other_char} on input. Possible values ['A', 'B', 'C']")
            }
        }      
    }
    score
}

fn main() {
    println!("Starting execution!\n");
    let start = Instant::now();

    let input = "./input.txt";
    
    let tournament_score = get_tournament_score(input);
    let real_tournament_score = get_real_tournament_score(input);

    println!("Your score on the tournament will be: {tournament_score}");
    println!("Your score on the tournament will be: {real_tournament_score}");

    let duration = start.elapsed();
    println!("\nTime elapsed in expensive_function() is: {:?}", duration);
}
