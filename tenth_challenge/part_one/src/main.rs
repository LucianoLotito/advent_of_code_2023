use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug, Clone, PartialEq, Copy)]

struct Position {
    line: usize,
    index: usize,
}

#[derive(Debug, Clone)]
struct Positioning {
    /*
        (line, position)
    */
    top_left: Option<Position>,
    top: Option<Position>,
    top_right: Option<Position>,
    left: Option<Position>,
    middle: Option<Position>,
    right: Option<Position>,
    bottom_left: Option<Position>,
    bottom: Option<Position>,
    bottom_right: Option<Position>,
}

impl Positioning {
    fn get_property(&self, property_name: &str) -> Option<Option<Position>> {
        match property_name {
            "top_left" => Some(self.top_left.to_owned()),
            "top" => Some(self.top.to_owned()),
            "top_right" => Some(self.top_right.to_owned()),
            "left" => Some(self.left.to_owned()),
            "middle" => Some(self.middle.to_owned()),
            "right" => Some(self.right.to_owned()),
            "bottom_left" => Some(self.bottom_left.to_owned()),
            "bottom" => Some(self.bottom.to_owned()),
            "bottom_right" => Some(self.bottom_right.to_owned()),
            _ => None,
        }
    }

    fn list_properties(&self) -> Vec<&str> {
        vec![
            "top_left",
            "top",
            "top_right",
            "left",
            "middle",
            "right",
            "bottom_left",
            "bottom",
            "bottom_right",
        ]
    }
}

fn main() {
    let file = File::open("../input.txt");
    let mut buf_reader = BufReader::new(file.unwrap());
    let mut contents = String::new();
    let _ = buf_reader.read_to_string(&mut contents);
    let lines: Vec<&str> = contents.lines().collect();
    let mut original_position = Positioning {
        top_left: None,
        top: None,
        top_right: None,
        left: None,
        middle: Some(Position { line: 0, index: 0 }),
        right: None,
        bottom_left: None,
        bottom: None,
        bottom_right: None,
    };
    let mut current_position = Positioning {
        top_left: None,
        top: None,
        top_right: None,
        left: None,
        middle: Some(Position { line: 0, index: 0 }),
        right: None,
        bottom_left: None,
        bottom: None,
        bottom_right: None,
    };

    for line in 0..lines.len() {
        let start = lines[line].find("S");
        if lines[line].find("S").is_some() {
            match set_position(
                Position {
                    line,
                    index: start.unwrap(),
                },
                contents.to_owned(),
            ) {
                Some(pos) => original_position = pos,
                None => {}
            };
        }

        while current_position.to_owned().middle != original_position.middle {
            println!("LOOP");
            match set_position(
                original_position.middle.to_owned().unwrap(),
                contents.to_owned(),
            ) {
                Some(new_pos) => {
                    current_position = new_pos;
                }
                None => (),
            }
            println!("{:?}", current_position);
        }
    }
}

fn set_position(center: Position, input: String) -> Option<Positioning> {
    let valid_positioning: HashMap<char, Vec<&str>> = HashMap::from([
        ('|', vec!["top", "bottom"]),
        ('-', vec!["left", "right"]),
        ('L', vec!["left", "bottom"]),
        ('J', vec!["bottom", "right"]),
        ('7', vec!["right", "top"]),
        ('F', vec!["left", "top"]),
        ('.', vec![]),
        ('S', vec![]),
        ('X', vec![]),
    ]);

    let mut lines: Vec<&str> = input.lines().collect();
    let mut mark_x = lines[center.line].to_owned();
    mark_x.replace_range(center.index..center.index + 1, "X");
    lines[center.line] = &mark_x;

    let mut current_position: Positioning;
    for line in 0..lines.len() {
        match lines[line].find("X") {
            Some(start) => {
                let is_top = line == 0;
                let is_right_top = start == lines[line].len();
                let is_left_top = start == 0;
                let is_bottom = line == lines.len();

                current_position = Positioning {
                    top_left: match is_left_top {
                        false => match is_top {
                            false => Some(Position {
                                line: line - 1,
                                index: start - 1,
                            }),
                            true => None,
                        },
                        true => None,
                    },
                    top_right: match is_right_top {
                        false => match is_top {
                            false => Some(Position {
                                line: line - 1,
                                index: start + 1,
                            }),
                            true => None,
                        },
                        true => None,
                    },
                    top: match is_top {
                        false => Some(Position {
                            line: line - 1,
                            index: start,
                        }),
                        true => None,
                    },
                    left: match is_left_top {
                        false => Some(Position {
                            line,
                            index: start - 1,
                        }),
                        true => None,
                    },
                    middle: Some(Position { line, index: start }),
                    right: match is_right_top {
                        false => Some(Position {
                            line,
                            index: start + 1,
                        }),
                        true => None,
                    },
                    bottom_left: match is_left_top {
                        false => match is_bottom {
                            false => Some(Position {
                                line: line + 1,
                                index: start - 1,
                            }),
                            true => None,
                        },
                        true => None,
                    },
                    bottom_right: match is_right_top {
                        false => match is_bottom {
                            false => Some(Position {
                                line: line + 1,
                                index: start + 1,
                            }),
                            true => None,
                        },
                        true => None,
                    },
                    bottom: match is_bottom {
                        false => Some(Position {
                            line: line + 1,
                            index: start,
                        }),
                        true => None,
                    },
                };

                for property in current_position.list_properties() {
                    let position = &current_position.get_property(property).unwrap().unwrap();
                    let position_char = lines[position.line].chars().nth(position.index).unwrap();
                    let is_valid_position = valid_positioning
                        .get(&position_char)
                        .unwrap()
                        .iter()
                        .any(|valid_pos| &property == valid_pos);
                }
                return Some(current_position);
            }
            None => {}
        }
    }
    None
}
