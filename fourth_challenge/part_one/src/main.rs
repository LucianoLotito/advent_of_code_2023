use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

use regex::{Captures, Match, Regex};

fn main() {
    let file = File::open("../input.txt");
    let mut buf_reader = BufReader::new(file.unwrap());
    let mut contents = String::new();
    let _ = buf_reader.read_to_string(&mut contents);

    let regex = Regex::new(r"\d+").unwrap();
    let mut lucky_nums: Vec<Vec<i32>> = Vec::new();
    let mut scratch_nums: Vec<Vec<i32>> = Vec::new();
    for line in contents.lines() {
        let formatted_line: Vec<&str> = line.split(": ").collect();
        let split_data = formatted_line
            .last()
            .unwrap()
            .split("|")
            .collect::<Vec<&str>>();

        let mut nums: Vec<i32> = Vec::new();
        regex
            .captures_iter(split_data.first().unwrap())
            .map(|winners| {
                winners
                    .iter()
                    .map(|lucky_num| {
                        nums.push(lucky_num.unwrap().as_str().parse::<i32>().unwrap());
                    })
                    .count();
            })
            .count();
        lucky_nums.push(nums);

        let mut card: Vec<i32> = Vec::new();
        regex
            .captures_iter(split_data.last().unwrap())
            .map(|winners| {
                winners
                    .iter()
                    .map(|card_num| {
                        card.push(card_num.unwrap().as_str().parse::<i32>().unwrap());
                    })
                    .count();
            })
            .count();
        scratch_nums.push(card);
    }

    let mut index = 0;
    let mut total_points = 0;
    scratch_nums
        .iter()
        .map(|read_card| {
            let mut points = 0;
            println!("{:?}", read_card);
            read_card
                .iter()
                .map(|card_number| {
                    if lucky_nums.get(index).unwrap().contains(card_number) {
                        points = if points == 0 { 1 } else { points * 2 };
                    }
                })
                .count();
            index += 1;
            total_points += points;
        })
        .count();
    println!("{}", total_points);
}
