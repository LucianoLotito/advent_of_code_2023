use std::collections::HashMap;
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
        let mut max_set = HashMap::from([("red", 0), ("blue", 0), ("green", 0)]);
        let clean_line: Vec<&str> = line.split(": ").collect();
        let lines_regex = Regex::new(r"(?U)(.*)(?:;|$)").unwrap();
        let _: Vec<()> = lines_regex
            .captures_iter(clean_line.last().unwrap())
            .map(|f| {
                let elements_regex = Regex::new(r"\d+\s+(red|green|blue)").unwrap();
                let _: Vec<()> = elements_regex
                    .captures_iter(f.get(0).unwrap().as_str())
                    .map(|s| {
                        let color_result = match Regex::new(r"red|green|blue")
                            .unwrap()
                            .find(s.get(0).unwrap().as_str())
                            .unwrap()
                            .as_str()
                        {
                            "red" => Some("red"),
                            "blue" => Some("blue"),
                            "green" => Some("green"),
                            _ => None,
                        };

                        let color_ammount = Regex::new(r"\d+")
                            .unwrap()
                            .find(s.get(0).unwrap().as_str())
                            .unwrap()
                            .as_str()
                            .parse::<i32>()
                            .unwrap();

                        if max_set.get(color_result.unwrap()).unwrap() < &color_ammount {
                            max_set.insert(color_result.unwrap(), color_ammount);
                        }
                    })
                    .collect();
            })
            .collect();
        let mut sub_total = 1;
        for (_, val) in max_set.iter() {
            sub_total *= val;
        }
        result += sub_total;
    }
    println!("{}", result);
}
