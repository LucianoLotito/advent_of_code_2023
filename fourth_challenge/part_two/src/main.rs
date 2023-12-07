use std::collections::BTreeMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

use regex::Regex;

fn main() {
    let file = File::open("../input.txt");
    let mut buf_reader = BufReader::new(file.unwrap());
    let mut contents = String::new();
    let _ = buf_reader.read_to_string(&mut contents);

    let regex = Regex::new(r"\d+").unwrap();
    let mut lucky_nums: BTreeMap<usize, Vec<i32>> = BTreeMap::new();
    let mut scratch_nums: BTreeMap<usize, Vec<i32>> = BTreeMap::new();
    let mut index = 1;
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
        let mut card: Vec<i32> = Vec::new();
        lucky_nums.insert(index, nums);

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
        scratch_nums.insert(index, card);

        index += 1;
    }

    let mut enumerated_cards: BTreeMap<usize, i32> = BTreeMap::new();
    scratch_nums
        .iter()
        .map(|x| enumerated_cards.insert(*x.0, 1))
        .count();

    for scratch in scratch_nums {
        for _ in 0..enumerated_cards.get(&scratch.0).unwrap().to_owned() {
            let mut index = 1;
            for numb in &scratch.1 {
                if lucky_nums.get(&scratch.0).unwrap().contains(&numb) {
                    let reps = enumerated_cards.get(&(scratch.0 + index)).unwrap();
                    enumerated_cards.insert(scratch.0 + index, reps + 1);
                    index += 1;
                }
            }
        }
    }
    let mut result = 0;
    enumerated_cards.iter().map(|nums| result += nums.1).count();
    println!("{}", result);
}
