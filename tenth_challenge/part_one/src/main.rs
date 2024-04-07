use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug)]
struct Possibilities {
    top: Option<usize>,
    bottom: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
}
struct Positioning {
    left: String,
    right: String,
    top: String,
    bottom: String,
}

#[derive(Debug, Clone, PartialEq)]
struct Position {
    line: usize,
    char: usize,
}
fn main() {
    let file = File::open("../input.txt");
    let mut buf_reader = BufReader::new(file.unwrap());
    let mut contents = String::new();
    let _ = buf_reader.read_to_string(&mut contents);
    let lines: Vec<&str> = contents.lines().collect();

    let initial_valid_positions = Positioning {
        left: "LF-".to_string(),
        right: "J7-".to_string(),
        top: "F7|".to_string(),
        bottom: "LJ|".to_string(),
    };

    let mut entry_point: Position = Position { line: 0, char: 0 };

    for line in 0..lines.len() {
        match lines[line].find('S') {
            Some(entry) => {
                entry_point = Position {
                    line: line.to_owned() as usize,
                    char: entry.to_owned() as usize,
                };
                break;
            }
            None => (),
        }
    }

    let mut current_pos = entry_point.clone();
    let mut previous_pos = entry_point.clone();

    let mut possibilities = Possibilities {
        left: None,
        right: None,
        top: None,
        bottom: None,
    };

    if current_pos.line as i8 - 1 >= 0 {
        if initial_valid_positions
            .top
            .find(
                lines[current_pos.line - 1]
                    .chars()
                    .nth(current_pos.char)
                    .unwrap()
                    .clone()
                    .to_string()
                    .as_str(),
            )
            .is_some()
        {
            println!("Im here");
            current_pos.line = current_pos.line - 1;
        }
    }
    if current_pos.char + 1 <= lines[0].len() {
        if initial_valid_positions
            .right
            .find(
                lines[current_pos.line]
                    .chars()
                    .nth(current_pos.char + 1)
                    .unwrap()
                    .clone()
                    .to_string()
                    .as_str(),
            )
            .is_some()
        {
            println!("Im here");
            current_pos.char = current_pos.char + 1;
        }
    }
    if current_pos.line + 1 <= lines.len() {
        if initial_valid_positions
            .bottom
            .find(
                lines[current_pos.line + 1]
                    .chars()
                    .nth(current_pos.char)
                    .unwrap()
                    .clone()
                    .to_string()
                    .as_str(),
            )
            .is_some()
        {
            println!("Im here");
            current_pos.line = current_pos.line + 1;
        }
    }
    if current_pos.char as i8 - 1 >= 0 {
        if initial_valid_positions
            .left
            .find(
                lines[current_pos.line]
                    .chars()
                    .nth(current_pos.char - 1)
                    .unwrap()
                    .clone()
                    .to_string()
                    .as_str(),
            )
            .is_some()
        {
            println!("Im here");
            current_pos.char = current_pos.char - 1;
        }
    }

    let valid_positions = Positioning {
        left: "7J-".to_string(),
        right: "FL-".to_string(),
        top: "JL|".to_string(),
        bottom: "F7|".to_string(),
    };

    let mut movements = 1;

    while (current_pos.line != entry_point.line) || (current_pos.char != entry_point.char) {
        let ourserlves = lines[current_pos.line]
            .chars()
            .nth(current_pos.char)
            .unwrap()
            .clone()
            .to_string();
        possibilities.top = None;
        possibilities.bottom = None;
        possibilities.left = None;
        possibilities.right = None;

        possibilities.top = valid_positions.top.find(ourserlves.clone().as_str());
        possibilities.right = valid_positions.right.find(ourserlves.clone().as_str());
        possibilities.bottom = valid_positions.bottom.find(ourserlves.clone().as_str());
        possibilities.left = valid_positions.left.find(ourserlves.clone().as_str());

        if possibilities.top.is_some() && (previous_pos.line != current_pos.line - 1) {
            previous_pos = current_pos.clone();
            current_pos.line = current_pos.line - 1;
        } else if possibilities.right.is_some() && previous_pos.char != current_pos.char + 1 {
            previous_pos = current_pos.clone();
            current_pos.char = current_pos.char + 1;
        } else if possibilities.bottom.is_some() && previous_pos.line != current_pos.line + 1 {
            previous_pos = current_pos.clone();
            current_pos.line = current_pos.line + 1;
        } else if possibilities.left.is_some() && previous_pos.char != current_pos.char - 1 {
            previous_pos = current_pos.clone();
            current_pos.char = current_pos.char - 1;
        }
        movements += 1;
    }
    println!("Furthest away: {}", movements / 2);
}
