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

fn parse_input(input: &str) -> Vec<Vec<u8>>  {
    let lines = read_lines(input)
        .unwrap_or_else(|_| panic!("File '{input}' should be accesible"));

    let mut tree_matrix: Vec<Vec<u8>> = Vec::new();
    let mut current_vec;

    for line in lines {
        if let Ok(line) = line {
            tree_matrix.push(Vec::new());
            current_vec = tree_matrix.last_mut().unwrap();
            for ch in line.chars() {
                current_vec.push(ch.to_digit(10).unwrap() as u8);
            }
        }
    }

    tree_matrix
}

fn _get_visible_trees(tree_matrix: &Vec<Vec<u8>>) -> u32 {
    let mut visible_trees: u32 = (tree_matrix.len()*2 + tree_matrix[0].len()*2 - 4) as u32;
    let mut set_idx_visible_trees: HashSet<(usize, usize)> = HashSet::new();

    let mut height_highest:u8 ;
    let mut idx_highest:usize;
    
    for (idx1, tree_line) in tree_matrix.iter().take(tree_matrix.len()-1).enumerate().skip(1) {

        // Add visible from left to right
        height_highest = tree_line[0];
        idx_highest = 0;
        for (idx2, tree_height) in tree_line.iter().take(tree_line.len()-1).enumerate() {
            if *tree_height >= height_highest {

                idx_highest = idx2; //Change the index so we know the rightest highest tree
                if *tree_height > height_highest {
                    height_highest = *tree_height;
                    set_idx_visible_trees.insert((idx1, idx2));
                    visible_trees += 1;
                }
            }
        }

        // Add visible from right to left
        height_highest = *tree_line.last().unwrap();
        for (idx2, tree_height) in tree_line.iter().enumerate().skip(idx_highest).rev().skip(1) {
            if *tree_height > height_highest {
                height_highest = *tree_height;
                if !set_idx_visible_trees.contains(&(idx1, idx2)) {
                    set_idx_visible_trees.insert((idx1, idx2));
                    visible_trees += 1;
                }
            }
        }
    }

    let mut current_tree_height;
    for idx2 in 1..tree_matrix[0].len()-1 {

        // Add visible from left to right
        height_highest = tree_matrix[0][idx2];
        idx_highest = 0;
        for idx1 in 1..tree_matrix.len()-1 {
            current_tree_height = tree_matrix[idx1][idx2];
            if current_tree_height >= height_highest {
                idx_highest = idx1; //Change the index so we know the most bottom highest tree
                if current_tree_height > height_highest  {
                    height_highest = current_tree_height;
                    if !set_idx_visible_trees.contains(&(idx1, idx2)) {
                        set_idx_visible_trees.insert((idx1, idx2));
                        visible_trees += 1;
                    }
                }
            }
        }

        // Add visible from right to left
        height_highest = tree_matrix[tree_matrix.len()-1][idx2];
        for idx1 in (idx_highest..tree_matrix.len()-1).rev() {
            current_tree_height = tree_matrix[idx1][idx2];
            if current_tree_height > height_highest  {
                height_highest = current_tree_height;
                if !set_idx_visible_trees.contains(&(idx1, idx2)) {
                    set_idx_visible_trees.insert((idx1, idx2));
                    visible_trees += 1;
                }
            }
        }
    }

    // TODO not sum 1
    visible_trees
}

fn get_visible_trees(input: &str) -> u32 {

    let tree_matrix = parse_input(input);

    return _get_visible_trees(&tree_matrix);
    
}


fn main() {
    println!("Starting execution!\n");
    let start = Instant::now();

    let input = "./input.txt";
    
    let visible_trees = get_visible_trees(input);

    println!("The number of visibles trees is {visible_trees}");

    let duration = start.elapsed();
    println!("\nTime elapsed is: {:?}", duration);
}
