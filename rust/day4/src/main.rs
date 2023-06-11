use std::{env, fs};

fn parse_pair(pair: &str) -> (usize, usize) {
    let mut numbers_iter = pair.split('-');
    let first = numbers_iter.next().unwrap().parse::<usize>().unwrap();
    let second = numbers_iter.next().unwrap().parse::<usize>().unwrap();
    (first, second)
}

fn is_number_inside(number: usize, pair: (usize, usize)) -> bool {
    return number >= pair.0 && number <= pair.1;
}

fn is_overlap(line: &str) -> bool {
    let mut pairs = line.split(',');
    let first = parse_pair(pairs.next().unwrap());
    let second = parse_pair(pairs.next().unwrap());
    if is_number_inside(first.0, second) || is_number_inside(first.1, second) {
        return true;
    } else if is_number_inside(second.0, first) || is_number_inside(second.1, first) {
        return true;
    } else {
        return false;
    }
}

fn is_inclusive(line: &str) -> bool {
    let mut pairs = line.split(',');
    let first = parse_pair(pairs.next().unwrap());
    let second = parse_pair(pairs.next().unwrap());
    if first.0 >= second.0 && first.1 <= second.1 {
        return true;
    } else if second.0 >= first.0 && second.1 <= first.1 {
       return true; 
    } else {
        return false;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("File name not provided");
    if let Ok(content) = fs::read_to_string(file_path) {
        let mut inclusive_pairs = 0;
        let mut overlapping_pairs = 0;
        for line in content.lines() {
            if is_inclusive(line) {
                inclusive_pairs += 1;
            }
            if is_overlap(line) {
                overlapping_pairs += 1;
            }
        }
        println!("Number of overlaping pairs: {overlapping_pairs}");
        println!("Number of inclusive pairs: {inclusive_pairs}");
    };
}
