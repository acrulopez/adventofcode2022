use std::time::Instant;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, PartialEq)]

enum ListItem {
    Number(u32),
    List(Box<Vec<ListItem>>)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_list(str_list: &str) -> Vec<ListItem> {
    // let mut root_vec:Vec<ListItem> = vec![];
    let mut vecs: Vec<Vec<ListItem>> = vec![];
    // let mut parent_list: Vec<&mut Vec<ListItem>>= vec![&mut list];
    // let mut current_list = &mut list;
    let mut number = String::new();

    for ch in str_list[0..str_list.len()-1].chars() {
        match ch {
            '[' => {
                vecs.push(vec![]);
                }
            ']' => {
                let mut vec = vecs.pop().unwrap();
                if !number.is_empty() {
                    vec.push(ListItem::Number(number.parse::<u32>().unwrap()));
                    number.clear();
                }
                vecs.last_mut().unwrap().push(ListItem::List(Box::new(vec)));
            }
            ' ' => {}
            ',' => {
                if !number.is_empty() {
                    vecs.last_mut().unwrap().push(ListItem::Number(number.parse::<u32>().unwrap()));
                    number.clear();
                }
            },
            digit => {
                number.push(digit);
            }
        }
    }

    if !number.is_empty() {
        vecs.last_mut().unwrap().push(ListItem::Number(number.parse::<u32>().unwrap()));
    }

    return vecs.pop().unwrap();
}

fn is_list_ordered(left: &Vec<ListItem>, right: &Vec<ListItem>) -> Option<bool> {

    for (i,j) in left.iter().zip(right) {
        if let (ListItem::Number(i), ListItem::Number(j)) = (i,j) {
            if i > j {return Some(false);}
            else if i < j {return Some(true);}
        }
        else if let (ListItem::Number(i), ListItem::List(j)) = (i,j) {
            if let Some(x) = is_list_ordered(&vec![ListItem::Number(*i)], j) {
                return Some(x);
            }
        }
        else if let (ListItem::List(i), ListItem::Number(j)) = (i,j) {
            if let Some(x) = is_list_ordered(i, &vec![ListItem::Number(*j)]) {
                return Some(x);
            }
        }
        else if let (ListItem::List(i), ListItem::List(j)) = (i,j) {
            if let Some(x) =  is_list_ordered(i, j) {
                return Some(x);
            }
        }
    }

    if left.len() > right.len() {return Some(false);}
    else if left.len() < right.len() {return Some(true)}

    None
}

fn is_pair_ordered(left: &str, right: &str) -> bool {

    let parsed_left = parse_list(left);
    let parsed_right = parse_list(right);

    return is_list_ordered(&parsed_left, &parsed_right).unwrap_or(true);

}

fn indices_sum(input: &str) -> u32 {
    let mut lines = read_lines(input)
        .unwrap_or_else(|_| panic!("File '{input}' should be accesible"));

    let mut idx_sum = 0;
    let mut it = 1;
    while let Some(Ok(line1)) = lines.next() {
        let line2 = lines.next().unwrap().unwrap();
        if is_pair_ordered(&line1, &line2) {
            idx_sum += it;
        }
        it += 1;
        lines.next(); // Remove the empty line
    }
    
    idx_sum
}

fn parse_to_number_list(list_items:Vec<ListItem>) -> Vec<i32> {

    let mut vec = vec![];

    if (&list_items).len() == 0 {
        return vec![-1];

    }
    for item in list_items {
        match item {
            ListItem::List(item) => {
                vec.append(&mut parse_to_number_list(*item))
            },
            ListItem::Number(item) => {vec.push(item as i32)},
        }
    }


    return vec;
}

fn decoder_key(input: &str) -> u32 {
    let mut lines = read_lines(input)
        .unwrap_or_else(|_| panic!("File '{input}' should be accesible"));

    let divider_packer1 = vec![2];
    let divider_packer2 = vec![6];
    
    let mut idx_divider_packer1 = 1;
    let mut idx_divider_packer2 = 2;
    
    while let Some(Ok(line1)) = lines.next() {
        let line2 = lines.next().unwrap().unwrap();
        
        let parsed_left = parse_to_number_list(parse_list(&line1));
        let parsed_right = parse_to_number_list(parse_list(&line2));

        if divider_packer1 > parsed_left {idx_divider_packer1+=1;}
        if divider_packer1 > parsed_right {idx_divider_packer1+=1;}
        if divider_packer2 > parsed_left {idx_divider_packer2+=1;}
        if divider_packer2 > parsed_right {idx_divider_packer2+=1;}

        lines.next(); // Remove the empty line
    }
    
    idx_divider_packer1*idx_divider_packer2
}

fn main() {
    println!("Starting execution!\n");
    let start = Instant::now();

    let input = "./input.txt";
    
    let idx_sum = indices_sum(input);

    println!("The sum of the indices is {}", idx_sum);

    let decoder_key = decoder_key(input);

    println!("The sum of the indices is {}", decoder_key);
    

    let duration = start.elapsed();
    println!("\nTime elapsed in expensive_function() is: {:?}", duration);
}
