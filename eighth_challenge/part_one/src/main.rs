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
            Regex::new(r"(([A-Z]){3}) = \((([A-Z]){3}), (([A-Z]){3})\)")
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
                })
        }
    });

    let mut exit = "AAA";
    let mut steps = 0;
    // While you havent found the exit, keep the loop going.
    while exit != "ZZZ" {
        // Iterate the instructions to know which direction to go to in every vector.
        instructions.iter().for_each(|inst| {
            // Get the instruction and compare it to the mapping and decide wether to use the left or right direction vector.
            match direction_mapping
                .get(inst.to_owned().to_string().as_str())
                .unwrap()
                // Once matched, change the value of the exit to the found vector in order to use its values as the next direction vector.
            {
                0 => {
                    exit = directions.get(exit).unwrap().0.as_str();
                }
                1 => {
                    exit = directions.get(exit).unwrap().1.as_str();
                }
                _ => (),
            }
            // Count the steps needed to get the exit vector (ZZZ)
            steps += 1;
        });
    }

    println!("{}", steps);
}
