use regex::Regex;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("../input.txt");
    let mut buf_reader = BufReader::new(file.unwrap());
    let mut contents = String::new();
    let _ = buf_reader.read_to_string(&mut contents);
    let mut races: Vec<u64> = Vec::new();
    let mut records: Vec<u64> = Vec::new();
    let mut total_possibilities: Vec<i32> = Vec::new();

    // Iterate every line of the input.
    for line in contents.trim().lines() {
        // Trim the line
        let trimmed_line = Regex::new(r"\s+")
            .unwrap()
            .replace_all(line, "")
            .to_owned()
            .to_string();
        // Apply a Regex to the line
        Regex::new(r"\d+")
            .unwrap()
            .captures_iter(&trimmed_line)
            .map(|value| {
                // If the line contains the string "Time:" then all of the results will be put into the
                // time vector
                if line.contains("Time: ") {
                    races.push(value.get(0).unwrap().as_str().parse::<u64>().unwrap());
                }
                // Else, the results will be put into the distance vector.
                else {
                    records.push(value.get(0).unwrap().as_str().parse::<u64>().unwrap());
                }
            })
            .count();
    }

    // Iterate each 1:1 race-record mapping
    for (race, record) in races.iter().zip(records.clone()) {
        let mut possibilities = 0;
        // Iterate the range of possible speeds to be used in the race, starting from 0
        for speed in 0..=race.clone() {
            // distance = time * speed.
            let time = race - speed;
            let distance = time * speed;
            // If the speed is able to beat the record distance in the set timer, add it to the possibilities.
            if distance > record {
                possibilities += 1;
            }
        }
        // Push the total number of possibilities into the possibilitites vector
        total_possibilities.push(possibilities);
    }

    // Calculate the total number of possibliities.
    let result = total_possibilities.iter().fold(1, |acc, &x| acc * x);

    println!("{}", result);
}
