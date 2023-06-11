use std::{env, fs};

const ALPHABET: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

fn calc_priority(byte_char: &u8) -> Option<usize> {
    for i in 0..ALPHABET.len() {
        let char = char::from_u32(*byte_char as u32).unwrap();
        let lowercase = char.clone().to_lowercase().collect::<Vec<_>>()[0];
        if lowercase == ALPHABET[i] {
            if char.is_uppercase() {
                return Some(i + 27);
            } else {
                return Some(i + 1);
            }
        }
    }
    None
}

fn find_common_item(bags: Vec<&str>) -> Option<usize> {
    let first_bag = bags[0].as_bytes();
    let second_bag = bags[1].as_bytes();
    let third_bag = bags[2].as_bytes();
    for first_char in first_bag {
        for second_char in second_bag {
            if second_char == first_char {
                for third_char in third_bag {
                    if third_char == first_char {
                        return calc_priority(first_char);
                    }
                }
            }
        }
    };
    None
}

fn check_bags(bag: &str) -> Option<usize> {
    let middle = bag.len() / 2;
    let first_pocket = bag.get(0..middle).unwrap();
    let second_pocket = bag.get(middle..bag.len()).unwrap();
    for byte_char in first_pocket.as_bytes() {
        for another_byte_char in second_pocket.as_bytes() {
            if byte_char == another_byte_char {
                return calc_priority(byte_char);
            }
        }
    }

    return None;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("Need a file path");
    if let Ok(content) = fs::read_to_string(file_path) {
        let same_items: Vec<usize> = content.lines().fold(vec![], |mut items, bag| {
            if let Some(item_score) = check_bags(bag) {
                items.push(item_score)
            }
            return items;
        });
        let mut i = 0;
        let mut common_items: Vec<usize> = vec![];
        while i < content.lines().count() {
            if let Some(three_bags) = content.lines().collect::<Vec<&str>>().get(i..i+3) {
                if let Some(item_score) = find_common_item(three_bags.to_vec()) {
                    common_items.push(item_score);
                }
            }
            i += 3;
        }
        println!("Same items: {:?}", same_items);
        println!("Total score: {}", same_items.iter().sum::<usize>());
        println!("Common items: {:?}", common_items);
        println!("Common score: {}", common_items.iter().sum::<usize>())
    }
}
