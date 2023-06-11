use std::{fs, env};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("should provide file name");
    if let Ok(content) = fs::read_to_string(file_path) {
        let mut calories_vec: Vec<usize> = vec![0];
        content.lines().fold(0, |index, value| {
            if value.len() > 0 {
               calories_vec[index] += value.parse::<usize>().expect("can't parse");
               return index;
            } else {
                calories_vec.push(0);
                return index + 1;
            }
        });
        calories_vec.sort();
        calories_vec.reverse();
        if let Some(cals) = calories_vec.get(0) {
            println!("Largest pack: {}", cals);
        }
        if let Some(top_three) = calories_vec.get(0..3) {
            println!("Top three: {:#?}", top_three);
            println!("There calories: {}", top_three.iter().sum::<usize>());
        }
    }
}
