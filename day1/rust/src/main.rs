use std::time::Instant;
use std::fs;


fn main() {
    println!("Starting execution!\n");
    let start = Instant::now();

    let input = "../input.txt";
    let contents = fs::read_to_string(input)
        .expect(&format!("Cannot open file {input}"));

    let duration = start.elapsed();
    println!("\nTime elapsed in expensive_function() is: {:?}", duration);
}
