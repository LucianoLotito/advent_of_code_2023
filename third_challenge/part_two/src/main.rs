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

    let chars_regex = Regex::new(r"\*").unwrap();
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
    let mut causer: HashMap<String, (Match, Vec<i32>)> = HashMap::new();
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
                                            let z = cr.start().to_string()
                                                + cr.end().to_string().as_str()
                                                + (num_pos.0 - &1).to_string().as_str();
                                            let _ = match causer.get(&z) {
                                                None => causer.insert(
                                                    z,
                                                    (
                                                        *cr,
                                                        vec![(nm.as_str().parse::<i32>().unwrap())],
                                                    ),
                                                ),
                                                Some(t) => {
                                                    let mut x = t.1.clone();
                                                    x.append(&mut vec![nm
                                                        .as_str()
                                                        .parse::<i32>()
                                                        .unwrap()]);
                                                    causer.insert(z, (*cr, x))
                                                }
                                            };
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
                                        let z = cr.start().to_string()
                                            + cr.end().to_string().as_str()
                                            + num_pos.0.to_string().as_str();
                                        let _ = match causer.get(&z) {
                                            None => causer.insert(
                                                z,
                                                (*cr, vec![(nm.as_str().parse::<i32>().unwrap())]),
                                            ),
                                            Some(t) => {
                                                let mut x = t.1.clone();
                                                x.append(&mut vec![nm
                                                    .as_str()
                                                    .parse::<i32>()
                                                    .unwrap()]);
                                                causer.insert(z, (*cr, x))
                                            }
                                        };
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
                                        let z = cr.start().to_string()
                                            + cr.end().to_string().as_str()
                                            + (num_pos.0 + &1).to_string().as_str();
                                        let _ = match causer.get(&z) {
                                            None => causer.insert(
                                                z,
                                                (*cr, vec![(nm.as_str().parse::<i32>().unwrap())]),
                                            ),
                                            Some(t) => {
                                                let mut x = t.1.clone();
                                                x.append(&mut vec![nm
                                                    .as_str()
                                                    .parse::<i32>()
                                                    .unwrap()]);
                                                causer.insert(z, (*cr, x))
                                            }
                                        };
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
    // println!("{:?}", causer);
    causer
        .iter()
        .map(|line| {
            if line.1 .1.len() == 2 {
                let mut sub_total = 1;
                line.1
                     .1
                    .iter()
                    .map(|element| {
                        sub_total *= element;
                    })
                    .count();
                result += sub_total;
            }
        })
        .count();
    println!("{}", result);
}
