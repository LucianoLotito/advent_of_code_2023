use regex::Regex;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("../input.txt");
    let mut buf_reader = BufReader::new(file.unwrap());
    let mut contents = String::new();
    let _ = buf_reader.read_to_string(&mut contents);

    let mut histories: Vec<Vec<i32>> = vec![];

    // Parse input data.
    contents.lines().into_iter().for_each(|line| {
        let mut history: Vec<i32> = vec![];
        Regex::new(r"-?\d+")
            .unwrap()
            .captures_iter(line)
            .map(|digit| {
                history.push(
                    digit
                        .get(0)
                        .unwrap()
                        .as_str()
                        .to_owned()
                        .parse::<i32>()
                        .unwrap(),
                )
            })
            .count();
        histories.push(history.to_owned());
    });

    // Store each history element and its extrapolation calculations, and result.
    let mut histories_extrapolation: Vec<Vec<Vec<i32>>> = vec![];
    for history in histories {
        // Take the current history element and clone it to manipulate it.
        let mut history_element: Vec<i32> = history.clone();
        // Add the current history element as part of the extrapolation.
        let mut history_element_extrapolation: Vec<Vec<i32>> = vec![history.to_owned()];
        // Iterate the history element until its sum is 0
        while !history_element.iter().all(|elmt| elmt == &0) {
            let mut extrapolation_element: Vec<i32> = vec![];
            // Iterate each element in the history element and try to calculate the difference between it and its successor,
            // then, add it to the extrapolation element
            for i in 0..history_element.len() {
                match history_element.get(i) {
                    None => (),
                    Some(element) => match history_element.get(i + 1) {
                        None => (),
                        Some(second_element) => {
                            extrapolation_element
                                .push((second_element.to_owned() - element.to_owned()) as i32);
                        }
                    },
                }
            }
            // Add the recently created extrapolation element to the history element extrapolation.
            if extrapolation_element.len() > 0 {
                history_element_extrapolation.push(extrapolation_element.to_owned());
            } else {
                history_element_extrapolation.push(vec![0]);
            }
            // change the current value of the history element for the new values, in order to iterate through the newly calculated values.
            history_element = extrapolation_element.to_owned();
        }
        histories_extrapolation.push(history_element_extrapolation);
    }

    let mut result = 0;
    let mut counter = 1;
    histories_extrapolation
        .iter_mut()
        .for_each(|element_extrapolation| {
            let cloned = element_extrapolation.clone();
            for extrapolation in (0..cloned.len() - 1).rev() {
                let current_element = element_extrapolation.get(extrapolation).unwrap().to_owned();
                match element_extrapolation.get_mut(extrapolation - 1) {
                    None => (),
                    Some(previous_element) => {
                        let extrapolated_value = current_element.iter().last().unwrap()
                            + previous_element.iter().last().unwrap();
                        previous_element.push(extrapolated_value);
                        if extrapolation == 1 {
                            counter += 1;
                            result += extrapolated_value;
                            break;
                        }
                    }
                }
            }
        });
    println!("{}", result);
}
