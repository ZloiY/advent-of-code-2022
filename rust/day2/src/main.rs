use std::{env, fs};

fn calc_score(symbol_e: &str, symbol_p: &str, solved_games: &mut Vec<Vec<usize>>) -> Option<usize> {
    match symbol_e {
        "A" => {
            let a_games = solved_games.get(0);
            match symbol_p {
                "X" => {
                    if let Some(solution) = Option::flatten(a_games.map(|sub_vec| sub_vec.get(0))) {
                        if *solution > 0 {
                            return Some(*solution);
                        } else {
                            solved_games[0][0] = 1 + 3;
                            return Some(1 + 3);
                        };
                    }
                    return None;
                },
                "Y" => {
                    if let Some(solution) = Option::flatten(a_games.map(|sub_vec| sub_vec.get(1))) {
                        if *solution > 0 {
                            return Some(*solution);
                        } else {
                            solved_games[0][1] = 2 + 6;
                            return  Some(2 + 6);
                        };
                    }
                    return None;
                },
                "Z" => {
                    if let Some(solution) = Option::flatten(a_games.map(|sub_vec| sub_vec.get(2))) {
                        if *solution > 0 {
                            return Some(*solution);
                        } else {
                            solved_games[0][2] = 3 + 0;
                            return Some(3 + 0);
                        };
                    }
                    return None;
                }
                _ => None
            }
        }
        "B" =>  {
            let b_games = solved_games.get(1);
            match symbol_p {
                "X" => {
                    if let Some(solution) = Option::flatten(b_games.map(|sub_vec| sub_vec.get(0))) {
                        if *solution > 0 {
                            return Some(*solution);
                        } else {
                            solved_games[1][0] = 1 + 0;
                            return Some(1 + 0)
                        };
                    }
                    None
                },
                "Y" => {
                    if let Some(solution) = Option::flatten(b_games.map(|sub_vec| sub_vec.get(1))) {
                        if *solution > 0 {
                            return Some(*solution);
                        } else {
                            solved_games[1][1] = 2 + 3;
                            return Some(2 + 3);
                        };
                    }
                    return None;
                },
                "Z" => {
                    if let Some(solution) = Option::flatten(b_games.map(|sub_vec| sub_vec.get(2))) {
                        if *solution > 0 {
                            return Some(*solution);
                        } else {
                            solved_games[1][2] = 3 + 6;
                            return Some(3 + 6);
                        };
                    }
                    return None;
                }
                _ => None
            }
        },
        "C" => {
            let c_games = solved_games.get(2);
            match symbol_p {
                "X" => {
                    if let Some(solution) = Option::flatten(c_games.map(|sub_vec| sub_vec.get(0))) {
                        if *solution > 0 {
                            return Some(*solution);
                        } else {
                            solved_games[2][0] = 1 + 6;
                            return Some(1 + 6);
                        };
                    }
                    return None;
                },
                "Y" => {
                    if let Some(solution) = Option::flatten(c_games.map(|sub_vec| sub_vec.get(1))) {
                        if *solution > 0 {
                            return Some(*solution);
                        } else {
                            solved_games[2][1] = 2 + 0;
                            return Some(2 + 0);
                        };
                    }
                    return None;
                },
                "Z" => {
                    if let Some(solution) = Option::flatten(c_games.map(|sub_vec| sub_vec.get(2))) {
                        if *solution > 0 {
                            return Some(*solution);
                        } else {
                            solved_games[2][2] = 3 + 3;
                            return Some(3 + 3);
                        };
                    }
                    return None;
                }
                _ => None
            }
    },
        _ => None,
    }
}

fn calc_score_v2(symbol_e: &str, symbol_p: &str, solved_games: &mut Vec<Vec<usize>>) -> Option<usize> {
    match symbol_e {
        "A" => {
            let a_games = solved_games.get(0);
            match symbol_p {
                "X" => {
                    if let Some(solution) = Option::flatten(a_games.map(|sub_vec| sub_vec.get(0))) {
                        if *solution > 0 {
                            return Some(*solution);
                        } else {
                            solved_games[0][0] = 3 + 0;
                            return Some(3 + 0);
                        };
                    }
                    return None;
                },
                "Y" => {
                    if let Some(solution) = Option::flatten(a_games.map(|sub_vec| sub_vec.get(1))) {
                        if *solution > 0 {
                            return Some(*solution);
                        } else {
                            solved_games[0][1] = 1 + 3;
                            return  Some(1 + 3);
                        };
                    }
                    return None;
                },
                "Z" => {
                    if let Some(solution) = Option::flatten(a_games.map(|sub_vec| sub_vec.get(2))) {
                        if *solution > 0 {
                            return Some(*solution);
                        } else {
                            solved_games[0][2] = 2 + 6;
                            return Some(2 + 6);
                        };
                    }
                    return None;
                }
                _ => None
            }
        }
        "B" =>  {
            let b_games = solved_games.get(1);
            match symbol_p {
                "X" => {
                    if let Some(solution) = Option::flatten(b_games.map(|sub_vec| sub_vec.get(0))) {
                        if *solution > 0 {
                            return Some(*solution);
                        } else {
                            solved_games[1][0] = 1 + 0;
                            return Some(1 + 0)
                        };
                    }
                    None
                },
                "Y" => {
                    if let Some(solution) = Option::flatten(b_games.map(|sub_vec| sub_vec.get(1))) {
                        if *solution > 0 {
                            return Some(*solution);
                        } else {
                            solved_games[1][1] = 2 + 3;
                            return Some(2 + 3);
                        };
                    }
                    return None;
                },
                "Z" => {
                    if let Some(solution) = Option::flatten(b_games.map(|sub_vec| sub_vec.get(2))) {
                        if *solution > 0 {
                            return Some(*solution);
                        } else {
                            solved_games[1][2] = 3 + 6;
                            return Some(3 + 6);
                        };
                    }
                    return None;
                }
                _ => None
            }
        },
        "C" => {
            let c_games = solved_games.get(2);
            match symbol_p {
                "X" => {
                    if let Some(solution) = Option::flatten(c_games.map(|sub_vec| sub_vec.get(0))) {
                        if *solution > 0 {
                            return Some(*solution);
                        } else {
                            solved_games[2][0] = 2 + 0;
                            return Some(2 + 0);
                        };
                    }
                    return None;
                },
                "Y" => {
                    if let Some(solution) = Option::flatten(c_games.map(|sub_vec| sub_vec.get(1))) {
                        if *solution > 0 {
                            return Some(*solution);
                        } else {
                            solved_games[2][1] = 3 + 3;
                            return Some(3 + 3);
                        };
                    }
                    return None;
                },
                "Z" => {
                    if let Some(solution) = Option::flatten(c_games.map(|sub_vec| sub_vec.get(2))) {
                        if *solution > 0 {
                            return Some(*solution);
                        } else {
                            solved_games[2][2] = 1 + 6;
                            return Some(1 + 6);
                        };
                    }
                    return None;
                }
                _ => None
            }
    },
        _ => None,
    }

}

fn main() {
    let mut solved_games_v2: Vec<Vec<usize>> = vec![];
    let mut solved_games: Vec<Vec<usize>> = vec![];
    for i in 0..3 {
        solved_games.push(vec![]);
        solved_games_v2.push(vec![]);
        for _ in 0..3 {
            solved_games[i].push(0);
            solved_games_v2[i].push(0);
        }
    }
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("Need file name");
    if let Ok(content) = fs::read_to_string(file_path) {
        let mut played_games: Vec<usize> = vec![];
        let mut played_games_v2: Vec<usize> = vec![];
        for game in content.lines() {
            let moves: Vec<_> = game.split(' ').collect();
            if let Some(score) = calc_score(moves[0], moves[1], solved_games.as_mut()) {
                played_games.push(score);
            }
            if let Some(score) = calc_score_v2(moves[0], moves[1], solved_games_v2.as_mut()) {
                played_games_v2.push(score);
            }
        };
        //println!("Games scores: {:?}", played_games);
        println!("Total score: {}", played_games.iter().sum::<usize>());
        //println!("Games scores v2: {:?}", played_games_v2);
        println!("Total score v2: {}", played_games_v2.iter().sum::<usize>());
    }
}
