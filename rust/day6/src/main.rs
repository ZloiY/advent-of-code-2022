use std::{env, fs};

fn search_message(signal: &str) -> usize {
    let (left, right) = signal.split_at(14);
    let mut range: [char; 14] = [' '; 14];
    let mut index = 0;
    for char in left.chars().take(14).collect::<Vec<char>>() {
        range[index] = char;
        index += 1;
    };
    let mut i = 0;
    for char in right.chars() {
        if range.len() == 14 {
            let mut breakable = false;
            for x in 0..14 {
                for y in (x + 1)..14 {
                    if range[x] == range[y] {
                        let mut z = 0;
                        while z < 13 {
                            range[z] = range[z+1];
                            z += 1;
                        }
                        breakable = true;
                        break;
                    }
                }
                if breakable {
                    break;
                }
            }
            if !breakable {
                return i + 14;
            }
        }
        range[13] = char;
        i += 1;
    }
    return 0;
}

fn search_marker(signal: &str) -> usize {
    let (left, right) = signal.split_at(4);
    let new_left: Vec<char> = left.chars().take(4).collect();
    let mut range: [char; 4] = [new_left[0], new_left[1], new_left[2], new_left[3]];
    let mut i = 0;
    for char in right.chars() {
        if range.len() == 4 {
            let mut breakable = false;
            for x in 0..4 {
                for y in (x + 1)..4 {
                    if range[x] == range[y] {
                        range[0] = range[1];
                        range[1] = range[2];
                        range[2] = range[3];
                        breakable = true;
                        break;
                    }
                }
                if breakable {
                    break;
                }
            }
            if !breakable {
                return i + 4;
            }
        }
        range[3] = char;
        i += 1;
    }
    return 0;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("Need file path");
    if let Ok(content) = fs::read_to_string(file_path) {
        for line in content.lines() {
            let marker_start = search_marker(line);
            let message_start = search_message(line);
            println!("Message start at postion: {message_start}");
            println!("Marker start at position: {marker_start}");
        }
    }
}
