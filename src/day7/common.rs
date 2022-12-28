use crate::utils;
use std::collections;
use std::fs;

pub enum RelativeDir {
    Root,
    Parent,
    Child(String),
}

pub enum Line {
    ChangeDir(RelativeDir),
    List,
    Directory(String),
    File(u32, String),
}

pub struct Directory {
    children: collections::HashMap<String, Node>,
}

struct File {
    size: u32,
}

enum Node {
    Directory(Directory),
    File(File),
}

fn create_file(current_state: &mut Directory, current_path: &[String], name: String, size: u32) {
    if current_path.is_empty() {
        current_state
            .children
            .insert(name, Node::File(File { size: size }));
    } else {
        let dir_name = &current_path[0];
        match current_state.children.get_mut(dir_name) {
            Some(ref mut e) => {
                if let Node::Directory(ref mut child) = e {
                    create_file(child, &current_path[1..], name, size)
                }
            }
            None => {
                let mut new_child = Directory {
                    children: collections::HashMap::new(),
                };
                create_file(&mut new_child, &current_path[1..], name, size);
                current_state
                    .children
                    .insert(dir_name.clone(), Node::Directory(new_child));
            }
        }
    }
}

fn change_directory(current_path: &mut Vec<String>, change: &RelativeDir) {
    match change {
        RelativeDir::Root => {
            current_path.clear();
        }
        RelativeDir::Parent => {
            current_path.pop();
        }
        RelativeDir::Child(name) => {
            current_path.push(name.clone());
        }
    }
}

fn execute_line(line: &Line, current_state: &mut Directory, current_path: &mut Vec<String>) {
    match line {
        Line::ChangeDir(dir) => {
            change_directory(current_path, dir);
        }
        Line::List => {}
        Line::Directory(_) => {}
        Line::File(size, name) => {
            create_file(current_state, current_path, name.clone(), *size);
        }
    }
}

pub fn directories_sizes(state: &Directory, results_so_far: &mut Vec<(String, u32)>) -> u32 {
    let mut result = 0;
    for (name, node) in state.children.iter() {
        match node {
            Node::File(f) => result += f.size,
            Node::Directory(d) => {
                let child_result = directories_sizes(d, results_so_far);
                results_so_far.push((name.clone(), child_result));
                result += child_result;
            }
        }
    }
    result
}

pub fn create_filesystem(lines: &Vec<Line>) -> Directory {
    let mut state = Directory {
        children: collections::HashMap::new(),
    };
    let mut path = vec![];
    for line in lines.iter() {
        execute_line(line, &mut state, &mut path);
    }
    state
}

fn parse_line(line: &str) -> Result<Line, Box<dyn std::error::Error>> {
    let parts = line.split_whitespace().collect::<Vec<_>>();
    if parts[0] == "$" {
        if parts[1] == "cd" {
            if parts[2] == "/" {
                Ok(Line::ChangeDir(RelativeDir::Root))
            } else if parts[2] == ".." {
                Ok(Line::ChangeDir(RelativeDir::Parent))
            } else {
                Ok(Line::ChangeDir(RelativeDir::Child(parts[2].to_string())))
            }
        } else if parts[1] == "ls" {
            Ok(Line::List)
        } else {
            Err(Box::new(utils::ParseInputError))
        }
    } else {
        if parts[0] == "dir" {
            Ok(Line::Directory(parts[1].to_string()))
        } else {
            Ok(Line::File(parts[0].parse()?, parts[1].to_string()))
        }
    }
}

pub fn read(filename: &str) -> Result<Vec<Line>, Box<dyn std::error::Error>> {
    fs::read_to_string(filename)?
        .lines()
        .map(parse_line)
        .collect()
}
