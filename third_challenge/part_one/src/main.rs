use core::num;
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::ops::Range;

use regex::{Match, Regex};

fn main() {
    let file = File::open("../input.txt");
    let mut buf_reader = BufReader::new(file.unwrap());
    let mut contents = String::new();
    let _ = buf_reader.read_to_string(&mut contents);

    let chars_regex = Regex::new(r"[^\w.\n]").unwrap();
    let numbs_regex = Regex::new(r"\d+").unwrap();

    let mut chars_positions: HashMap<usize, Vec<Match>> = HashMap::new();
    let mut numbs_positions: HashMap<usize, Vec<Match>> = HashMap::new();
    let mut index = 0;
    for line in contents.lines() {
        let numb: Vec<Match> = numbs_regex
            .captures_iter(line)
            .map(|f| f.get(0).unwrap())
            .collect();
        let char: Vec<Match> = chars_regex
            .captures_iter(line)
            .map(|f| f.get(0).unwrap())
            .collect();

        if !numb.is_empty() {
            numbs_positions.insert(index, numb);
        }

        if !char.is_empty() {
            chars_positions.insert(index, char);
        }

        index += 1;
    }
    let mut result = 0;
    numbs_positions
        .iter()
        .map(|num_pos| {
            num_pos
                .1
                .iter()
                .map(|nm| {
                    if num_pos.0 > &0 {
                        chars_positions
                            .get(&(num_pos.0 - &1))
                            .iter()
                            .map(|char| {
                                char.iter()
                                    .map(|cr| {
                                        let rng = Range {
                                            start: if nm.start() > 0 { nm.start() - 1 } else { 0 },
                                            end: nm.end() + 1,
                                        };
                                        if rng.contains(&cr.start()) {
                                            result += nm.as_str().parse::<i32>().unwrap();

                                            println!("{:?} is contained becasue of {:?}", nm, cr);
                                        }
                                    })
                                    .count();
                            })
                            .count();
                    }
                    chars_positions
                        .get(num_pos.0)
                        .iter()
                        .map(|char| {
                            char.iter()
                                .map(|cr| {
                                    let rng = Range {
                                        start: if nm.start() > 0 { nm.start() - 1 } else { 0 },
                                        end: nm.end() + 1,
                                    };
                                    if rng.contains(&cr.start()) {
                                        result += nm.as_str().parse::<i32>().unwrap();

                                        println!("{:?} is contained becasue of {:?}", nm, cr);
                                    }
                                })
                                .count();
                        })
                        .count();
                    chars_positions
                        .get(&(num_pos.0 + &1))
                        .iter()
                        .map(|char| {
                            char.iter()
                                .map(|cr| {
                                    let rng = Range {
                                        start: if nm.start() > 0 { nm.start() - 1 } else { 0 },
                                        end: nm.end() + 1,
                                    };
                                    if rng.contains(&cr.start()) {
                                        result += nm.as_str().parse::<i32>().unwrap();

                                        println!("{:?} is contained becasue of {:?}", nm, cr);
                                    }
                                })
                                .count();
                        })
                        .count();
                })
                .count();
        })
        .count();
    println!("{}", result);
}
