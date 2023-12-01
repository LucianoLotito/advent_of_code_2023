use std::fs::File;
use std::io::{prelude::*, BufReader};

use regex::Regex;

fn main() {
    let file = File::open("../input.txt");
    let mut buf_reader = BufReader::new(file.unwrap());
    let mut contents = String::new();
    let _ = buf_reader.read_to_string(&mut contents);
    let mut result = 0;

    for line in contents.lines() {
        let re = Regex::new(r"\d").unwrap();
        let data: Vec<&str> = re
            .captures_iter(line)
            .map(|f| {
                let a = f.get(0).unwrap().as_str();
                a
            })
            .collect();
        let first_element = match data.first() {
            None => None,
            Some(x) => Some(x),
        };
        let last_element = match data.last() {
            None => None,
            Some(x) => Some(x),
        };
        let extracted_numbers =
            first_element.unwrap_or(&"").to_string() + last_element.unwrap_or(&"");
        result += extracted_numbers.parse::<u32>().unwrap()
    }
    println!("{}", result);
}
