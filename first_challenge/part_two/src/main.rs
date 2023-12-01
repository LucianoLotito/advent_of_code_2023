use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::str::FromStr;

use regex::Regex;

fn main() {
    let file = File::open("../input.txt");
    let mut buf_reader = BufReader::new(file.unwrap());
    let mut contents = String::new();
    let _ = buf_reader.read_to_string(&mut contents);
    let mut result = 0;
    let num_dict: HashMap<&str, &str> = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    for line in contents.lines() {
        let re = Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine").unwrap();
        let rev_re = Regex::new(r"\d|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin").unwrap();
        let rev_line: String = line.chars().rev().collect();

        let first_data = re.find(line).map(|x| x.as_str()).unwrap_or("");
        let last_data = rev_re.find(&rev_line).map(|x| x.as_str()).unwrap_or("");
        let reversed_data: String = String::from_str(last_data).unwrap().chars().rev().collect();

        let extracted_numbers = num_dict.get(first_data).unwrap_or(&first_data).to_string()
            + num_dict
                .get(reversed_data.as_str())
                .unwrap_or(&reversed_data.as_str());

        result += extracted_numbers.parse::<u32>().unwrap();
    }
    println!("{}", result);
}
