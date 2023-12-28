use num::Integer;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("../input.txt");
    let mut buf_reader = BufReader::new(file.unwrap());
    let mut contents = String::new();
    let _ = buf_reader.read_to_string(&mut contents);

    // Use the direction mapping to know which tuple to use when going left or right.
    let direction_mapping = HashMap::from([("L", 0), ("R", 1)]);
    let mut instructions: Vec<char> = Vec::new();
    let mut starting_positions: Vec<String> = Vec::new();
    let mut directions: HashMap<
        /*Starting point*/ String,
        (
            /*Left indication*/ String,
            /*Right indication*/ String,
        ),
    > = HashMap::new();

    // Parse input data.
    contents.lines().into_iter().for_each(|line| {
        // If the instructions are not yet set, go in here and find them out in the input
        if instructions.is_empty() {
            match Regex::new(r"([A-Z])+").unwrap().find(line) {
                None => (),
                Some(instruct) => {
                    instruct
                        .as_str()
                        .trim()
                        .chars()
                        .into_iter()
                        .for_each(|instuct| instructions.push(instuct));
                }
            }
        }
        // Else, find out the directions vector.
        else {
            Regex::new(r"(([A-Z|0-9]){3}) = \((([A-Z|0-9]){3}), (([A-Z|0-9]){3})\)")
                .unwrap()
                .captures_iter(line.trim())
                .for_each(|instr| {
                    directions.insert(
                        instr.get(1).unwrap().as_str().to_owned().to_string(),
                        (
                            instr.get(3).unwrap().as_str().to_owned().to_string(),
                            instr.get(5).unwrap().as_str().to_owned().to_string(),
                        ),
                    );
                    if Regex::new(r"[A-Z|0-9]{2}A")
                        .unwrap()
                        .is_match(instr.get(1).unwrap().as_str())
                    {
                        starting_positions
                            .push(instr.get(1).unwrap().as_str().to_owned().trim().to_string());
                    }
                });
        }
    });

    let mut max_steps: Vec<u32> = vec![];
    let mut exit: String;
    // While you havent found the exit, keep the loop going.
    for start in starting_positions {
        let mut steps = 0;
        exit = start;
        while !Regex::new(r"[A-Z|0-9]{2}Z").unwrap().is_match(&exit) {
            // Iterate the instructions to know which direction to go to in every vector.
            for inst in instructions.iter() {
                // Get the instruction and compare it to the mapping and decide wether to use the left or right direction vector.
                match direction_mapping
            .get(inst.to_owned().to_string().as_str())
            .unwrap()
            // Once matched, change the value of the exit to the found vector in order to use its values as the next direction vector.
            {
                0 => {
                    exit = directions.get(&exit).unwrap().0.to_string();
                }
                1 => {
                    exit = directions.get(&exit).unwrap().1.to_string();
                }
                _ => (),
            }
                // Count the steps needed to get the exit vector (ZZZ)
                steps += 1;
                if Regex::new(r"[A-Z|0-9]{2}Z").unwrap().is_match(&exit) {
                    break;
                }
            }
        }
        max_steps.push(steps);
    }

    // Calculate the Lowest Common Multiple for the starting points in order to get the required
    // number of steps for all vectors to exit at the same time.
    let lcm_result = max_steps
        .iter()
        .fold(Some(1), |acc: Option<u64>, &num| match acc {
            Some(current_lcm) => Some(current_lcm.lcm(&(num as u64))),
            None => None,
        });

    match lcm_result {
        Some(result) => println!("The LCM of the vector is: {}", result),
        None => println!("Cannot find LCM of an empty vector"),
    }
}
