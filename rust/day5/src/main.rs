use std::{env, fs};

fn moving_creates(
    pick_from: usize,
    items_number: usize,
    put_to: usize,
    test_input: &mut [Vec<char>; 9],
) {
    let mut change_vec = &mut test_input[pick_from - 1];
    let mut picked_items: Vec<char> = vec![];
    for _ in 0..items_number {
        let item = change_vec.pop().unwrap();
        picked_items.push(item);
    }
    change_vec = &mut test_input[put_to - 1];
    for item in picked_items {
        change_vec.push(item)
    }
}

fn moving_creates_in_stack(
    pick_from: usize,
    items_number: usize,
    put_to: usize,
    test_input: &mut [Vec<char>; 3],
) {
    let mut change_vec = &mut test_input[pick_from - 1];
    let mut picked_items: Vec<char> = vec![];
    for _ in 0..items_number {
        let item = change_vec.pop().unwrap();
        picked_items.push(item);
    }
    picked_items.reverse();
    change_vec = &mut test_input[put_to - 1];
    for item in picked_items {
        change_vec.push(item)
    }
}

fn main() {
    let mut test_input: [Vec<char>; 3] = [vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
    /*let mut test_input: [Vec<char>; 9] = [
        vec!['D', 'L', 'J', 'R', 'V', 'G', 'F'],
        vec!['T', 'P', 'M', 'B', 'V', 'H', 'J', 'S'],
        vec!['V', 'H', 'M', 'F', 'D', 'G', 'P', 'C'],
        vec!['M', 'D', 'P', 'N', 'G', 'Q'],
        vec!['J', 'L', 'H', 'N', 'F'],
        vec!['N', 'F', 'V', 'Q', 'D', 'G', 'T', 'Z'],
        vec!['F', 'D', 'B', 'L'],
        vec!['M', 'J', 'B', 'S', 'V', 'D', 'N'],
        vec!['G', 'L', 'D'],
    ];*/
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("File path needed");
    if let Ok(content) = fs::read_to_string(file_path) {
        for line in content.lines() {
            let numbers: Vec<usize> = line
                .split(' ')
                .filter_map(|word| match word.parse::<usize>() {
                    Ok(number) => Some(number),
                    Err(_) => None,
                })
                .collect();
            let items_number = numbers[0];
            let pick_from = numbers[1];
            let put_to = numbers[2];
            moving_creates_in_stack(pick_from, items_number, put_to, &mut test_input);
            println!("{:?}", test_input);
        }
        for crates in test_input.iter() {
            println!("Top crate: {}", crates.last().unwrap());
        }
    }
}
