use std::time::Instant;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

impl Default for LocalFolder {
    fn default() -> LocalFolder {
        LocalFolder {
            files: Vec::<LocalFile>::new(),
            folders: Vec::<usize>::new(),
            parent_folder: None,
            // name: "/".to_string(),
        }
    }
}

struct LocalFile {
    size: u32,
    // name: String
}
struct LocalFolder {
    parent_folder: Option<usize>,
    files: Vec<LocalFile>,
    folders: Vec<usize>,
    // name: String
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_or_create_folder(folder_vec: &mut Vec<LocalFolder>, current_folder: usize, folder_name: &str) -> usize {
    match folder_name {
        ".." => {
            return folder_vec[current_folder].parent_folder.unwrap();
        }
        "/" => current_folder,
        "." => current_folder,
        _ => {
            let new_folder = LocalFolder {
                parent_folder: Some(current_folder), 
                // name: folder_name.to_string(),
                ..Default::default()
            };
            (*folder_vec).push(new_folder);
            let vec_last_index = folder_vec.len()-1;
            folder_vec[current_folder].folders.push(vec_last_index);
            return vec_last_index;
        }
    }
}

fn parse_input(input: &str) -> Vec<LocalFolder>  {
    let lines = read_lines(input)
        .unwrap_or_else(|_| panic!("File '{input}' should be accesible"));

    let mut folders: Vec<LocalFolder> = vec![LocalFolder {..Default::default()}];
    let mut current_folder: usize = 0;

    for line in lines {
        if let Ok(line) = line {
            let splitted_str:Vec<&str>  = line.split_whitespace().collect();
            match splitted_str[0] {
                "$" => {
                    if splitted_str[1] == "cd" {
                        current_folder = get_or_create_folder(
                            &mut folders, 
                            current_folder,
                            splitted_str[2]);
                    }
                },
                "dir" => (),
                number => {
                    folders[current_folder].files.push(
                        LocalFile { 
                            size: number.parse().unwrap(), 
                            // name: splitted_str[1].to_string()
                        }
                    )
                }
            }
        }
    }

    folders
}

fn _find_folder_smaller_than(folders: &Vec<LocalFolder>, current_folder:usize, smaller_than: u32) 
    -> (u32, Option<Vec<u32>>) {

        let mut size:u32 = 0;
        let mut smaller_sizes = Vec::new();

        for folder in &folders[current_folder].folders {
            let (subfolder_size, mut sub_smaller_sizes) = 
                _find_folder_smaller_than(folders, *folder, smaller_than);

            if let Some(sub_smaller_sizes) = &mut sub_smaller_sizes {
                smaller_sizes.append(sub_smaller_sizes);
            }
            size += subfolder_size;
        }

        for file in  &folders[current_folder].files {
            size += file.size;
        }

        if size <= smaller_than {
            smaller_sizes.push(size);
        }

        return (size, Some(smaller_sizes));
        
}

fn sum_sizes_smaller_than(folders: &Vec<LocalFolder>, smaller_than: u32) 
    -> u32 {

    let (_size, idx_vector) = 
        _find_folder_smaller_than(folders, 0, smaller_than);

    if let Some(idx_vector) = idx_vector {
        return idx_vector.iter().sum::<u32>();
    }
    0
}

fn folder_sizes(folders: &Vec<LocalFolder>, current_folder:usize) 
    -> (u32, Vec<u32>) {

        let mut size:u32 = 0;
        let mut sizes = Vec::new();

        for folder in &folders[current_folder].folders {
            let (subfolder_size, mut subfolder_sizes) = 
                folder_sizes(folders, *folder);

                sizes.append(&mut subfolder_sizes);

            size += subfolder_size;
        }

        for file in  &folders[current_folder].files {
            size += file.size;
        }

        sizes.push(size);

        return (size, sizes);
        
}

fn compute_sum_smaller_than(input: &str, smaller_than: u32) -> u32 {

    let folders = parse_input(input);

    let total_size = sum_sizes_smaller_than(&folders, smaller_than);

    total_size
    
}

fn smallest_folder_to_delete(input: &str, disk_space: u32, space_needed: u32) -> u32 {

    let folders = parse_input(input);

    let (root_size, folder_sizes) = folder_sizes(&folders, 0);

    let delete_size_needed = root_size - (disk_space - space_needed);

    let mut smallest_size = root_size;

    for size in folder_sizes {
        if size >= delete_size_needed && size <= smallest_size {
            smallest_size = size;
        }
    }

    smallest_size
}

fn main() {
    println!("Starting execution!\n");
    let start = Instant::now();

    let input = "./input.txt";
    
    let smaller_than_100000 = compute_sum_smaller_than(input, 100000);
    let smallest_to_delete = smallest_folder_to_delete(input, 70000000, 30000000);

    println!("The total size of folders smaller than 100000 is {smaller_than_100000}");
    println!("The smallest folder to delete has a size of {smallest_to_delete}");

    let duration = start.elapsed();
    println!("\nTime elapsed is: {:?}", duration);
}
