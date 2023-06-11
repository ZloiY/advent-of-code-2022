use std::{
    cell::{Ref, RefCell, RefMut},
    env, fs,
    rc::Rc, borrow::{BorrowMut, Borrow},
};

#[derive(Debug, Clone)]
enum FileSystem {
    Directory(Rc<RefCell<Directory>>),
    File(File),
}

#[derive(Debug)]
enum Command {
    PushItems,
    GoUp,
    OpenDir(String),
}

#[derive(Debug)]
enum ParseResult {
    Command(Command),
    File(File),
    Dir(String),
    None,
}

#[derive(Debug, Clone)]
struct Directory {
    parent: Option<Rc<RefCell<Directory>>>,
    name: String,
    children: Box<Vec<FileSystem>>,
}

#[derive(Debug, Clone)]
struct File {
    name: String,
    size: usize,
}

fn parse_command(line: &str) -> ParseResult {
    if '$' == line.chars().take(1).next().unwrap() {
        let mut chars = line.chars().skip(2).take(2);
        let mut command_string = String::with_capacity(2);
        command_string.push(chars.next().unwrap());
        command_string.push(chars.next().unwrap());
        return match command_string.as_str() {
            "cd" => {
                let dir_name = line.chars().skip(5).fold(String::new(), |mut state, char| {
                    state.push(char);
                    state
                });
                if dir_name.as_str() == ".." {
                    return ParseResult::Command(Command::GoUp);
                } else {
                    return ParseResult::Command(Command::OpenDir(dir_name));
                }
            }
            "ls" => ParseResult::Command(Command::PushItems),
            _ => ParseResult::None,
        };
    } else if line.chars().take(3).collect::<Vec<char>>()[..] == vec!['d', 'i', 'r'] {
        let dir_name = line.chars().skip(4).fold(String::new(), |mut state, char| {
            state.push(char);
            state
        });
        return ParseResult::Dir(dir_name);
    } else {
        let (size, name) = line.split_once(' ').unwrap();
        return ParseResult::File(File {
            name: name.to_string(),
            size: size.to_string().parse::<usize>().unwrap(),
        });
    }
}

fn fill_the_tree(commands_list: &String) -> &Directory {
    let file_system = Rc::new(RefCell::new(Directory {
        parent: None,
        name: String::from("/"),
        children: Box::new(Vec::new()),
    }));
    let mut cur_parent: Rc<RefCell<Directory>> = Rc::clone(&file_system);
    for line in commands_list.lines() {
        let parse_result = parse_command(line);
        match parse_result {
            ParseResult::Command(Command::OpenDir(dir_name)) => {
                let borrowed_val = Rc::clone(&cur_parent);
                if dir_name != String::from("/") {
                    let new_dir = borrowed_val.into_inner()
                        .children
                        .iter()
                        .find_map(|file| match file {
                            FileSystem::Directory(dir) => {
                                if dir.to_owned().into_inner().name == dir_name {
                                    Some(dir)
                                } else {
                                    None
                                }
                            }
                            _ => None,
                        })
                        .expect("Couldn't find dir");

                    cur_parent = Rc::clone(&new_dir.as_ref());
                }
            }
            ParseResult::Command(Command::PushItems) => {}
            ParseResult::Command(Command::GoUp) => {
                if let Some(parent) = cur_parent.into_inner().parent {
                    cur_parent = Rc::clone(&parent); 
                };
            }
            ParseResult::File(file) => {
                cur_parent.into_inner().children.push(FileSystem::File(file));
            }
            ParseResult::Dir(dir_name) => {
                //let new_dir = Directory::new(Some(Rc::clone(&cur_parent)), dir_name);
                //let rc_dir = Rc::new(&mut new_dir);
                //if let Some(mut_parent) = Rc::get_mut(&mut cur_parent) {
                //    println!("{:?}", mut_parent);
                //    mut_parent
                //        .children
                //        .push(FileSystem::Directory(Rc::new(new_dir)));
                //}
            }
            ParseResult::None => {}
        }
    }
    return file_system.into_inner().borrow();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("File name required");
    if let Ok(content) = fs::read_to_string(file_path) {
        let file_tree = fill_the_tree(&content);
        println!("{:#?}", file_tree);
    }
}
