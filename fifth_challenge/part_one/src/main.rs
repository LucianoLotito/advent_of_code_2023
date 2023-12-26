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

    let mut seeds: Vec<i64> = Vec::new();
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
            seeds = Regex::new(r"\d+")
                .unwrap()
                .captures_iter(line)
                .map(|seed| seed.get(0).unwrap().as_str().parse::<i64>().unwrap())
                .collect();
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

    let mut value: BTreeMap<i64, Vec<(String, i64)>> = BTreeMap::new();
    let keys = vec![
        "seed-to-soil",
        "soil-to-fertilizer",
        "fertilizer-to-water",
        "water-to-light",
        "light-to-temperature",
        "temperature-to-humidity",
        "humidity-to-location",
    ];
    let mut locations: Vec<i64> = Vec::new();

    let mut prev_map: i64;
    // Iterate through every found seed
    '_seed_loop: for element in seeds {
        prev_map = element;
        value.insert(element, vec![]);
        // For each seed, iterate through each mapping key
        'key_loop: for key in keys.to_owned() {
            let map_name = mapings.get(key).unwrap();
            // For every mapping key, iterate through each origin-destination range mapping
            '_mapping_loop: for (origin, destination) in map_name.0.iter().zip(map_name.1.iter()) {
                // If the origin mapping contains the current mapping value, enter this block of code and find its destination value
                if origin.contains(&prev_map) {
                    prev_map = destination.to_owned().min().unwrap()
                        + (origin.to_owned().min().unwrap() - prev_map).abs();
                    value
                        .entry(element)
                        .and_modify(|f| f.push((key.to_owned(), prev_map.to_owned())));
                    // value.push(
                    //     destination.to_owned().min().unwrap()
                    //         + (origin.to_owned().min().unwrap() - element).abs(),
                    // );
                    if key == "humidity-to-location" {
                        locations.push(prev_map);
                    }
                    continue 'key_loop;
                }
            }
            // After iterating through every mapping, if the mapping value was not contained in any range, insert the unaltered value as
            // a mapping result.
            value
                .entry(element)
                .and_modify(|f| f.push((key.to_owned(), prev_map.to_owned())));

            if key == "humidity-to-location" {
                locations.push(prev_map);
            }
        }
    }

    println!("{:?}", value);
    println!("{:?}", locations.iter().min());
}
