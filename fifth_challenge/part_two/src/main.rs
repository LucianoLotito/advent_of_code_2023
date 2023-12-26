use std::collections::BTreeMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::ops::Range;

use regex::Regex;

fn main() {
    let file = File::open("../input.txt");
    let mut buf_reader = BufReader::new(file.unwrap());
    let mut contents = String::new();
    let _ = buf_reader.read_to_string(&mut contents);

    let mut seed_ranges: Vec<Range<i64>> = Vec::new();
    let mut mapings: BTreeMap<
        /*Map title*/ String,
        (
            /*Origin*/ Vec<Range<i64>>,
            /*Destination*/ Vec<Range<i64>>,
        ),
    > = BTreeMap::new();
    let mut current_map_key = String::new();
    for line in contents.trim().lines() {
        if Regex::new(r"seeds: ").unwrap().find(line).is_some() {
            Regex::new(r"(\d+) (\d+)")
                .unwrap()
                .captures_iter(line)
                .map(|seed| {
                    let from = seed.get(1).unwrap().as_str().parse::<i64>().unwrap();
                    let to = from + seed.get(2).unwrap().as_str().parse::<i64>().unwrap();
                    seed_ranges.push(from..to);
                })
                .count();
            continue;
        }
        if let Some(captures) = Regex::new(r"(\w+)-to-(\w+) map:").unwrap().captures(line) {
            let from = captures.get(1).unwrap().as_str();
            let to = captures.get(2).unwrap().as_str();
            current_map_key = format!("{}-to-{}", from, to);
            mapings.insert(current_map_key.clone(), (vec![], vec![]));
        }

        let mut map_values: Vec<i64> = Vec::new();
        Regex::new(r"\d+")
            .unwrap()
            .captures_iter(line)
            .map(|maps| map_values.push(maps.get(0).unwrap().as_str().parse::<i64>().unwrap()))
            .count();

        if map_values.len() > 0 {
            mapings
                .entry(current_map_key.clone())
                .or_insert_with(|| (Vec::new(), Vec::new()))
                .0
                .push(
                    map_values[1].clone()
                        ..map_values[1].clone() + map_values.last().unwrap().clone(),
                );
            mapings
                .entry(current_map_key.clone())
                .or_insert_with(|| (Vec::new(), Vec::new()))
                .1
                .push(
                    map_values.first().unwrap().clone()
                        ..map_values.first().unwrap().clone() + map_values.last().unwrap().clone(),
                );
        }
    }

    let keys = vec![
        "humidity-to-location",
        "temperature-to-humidity",
        "light-to-temperature",
        "water-to-light",
        "fertilizer-to-water",
        "soil-to-fertilizer",
        "seed-to-soil",
    ];

    let mut value: i64 = 0;
    let mut prev_map: i64;

    // This loop is meant to find the aproximate range of the value. In order to improve efficiency.
    'until_found: loop {
        prev_map = value;
        '_key_loop: for key in keys.clone() {
            'rng_loop: for (origin, destination) in mapings
                .get(key)
                .unwrap()
                .to_owned()
                .0
                .iter()
                .zip(mapings.get(key).unwrap().to_owned().1)
            {
                if destination.contains(&prev_map) {
                    prev_map = origin.to_owned().min().unwrap()
                        + (destination.to_owned().min().unwrap() - prev_map).abs();
                    break 'rng_loop;
                }
            }
        }
        for s_rng in seed_ranges.clone() {
            if s_rng.contains(&prev_map) {
                break 'until_found;
            }
        }
        value += 1000000;
    }

    // Substract a million from the value, since it is guaranteed that the wanted value is less than the value returned
    // in the previous loop, and start iterating again.
    value -= 1000000;
    'until_found: loop {
        prev_map = value;
        '_key_loop: for key in keys.clone() {
            'rng_loop: for (origin, destination) in mapings
                .get(key)
                .unwrap()
                .to_owned()
                .0
                .iter()
                .zip(mapings.get(key).unwrap().to_owned().1)
            {
                if destination.contains(&prev_map) {
                    prev_map = origin.to_owned().min().unwrap()
                        + (destination.to_owned().min().unwrap() - prev_map).abs();
                    break 'rng_loop;
                }
            }
        }
        for s_rng in seed_ranges.clone() {
            if s_rng.contains(&prev_map) {
                println!("Value {} maps to seed range {:?}", value, s_rng);
                break 'until_found;
            }
        }
        value += 1;
    }
}
