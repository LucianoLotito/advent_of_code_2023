use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("../input.txt");
    let mut buf_reader = BufReader::new(file.unwrap());
    let mut contents = String::new();
    let _ = buf_reader.read_to_string(&mut contents);

    let mut hands: Vec<(
        /*Hand*/ String,
        /*Bet*/ u16,
        /*Strength*/ u8,
        /*Hand cards*/ Vec<(char, u8)>,
    )> = Vec::new();

    // Map the values of the cards.
    let card_values: HashMap<char, u8> = HashMap::from([
        ("A".parse::<char>().unwrap(), 14),
        ("K".parse::<char>().unwrap(), 13),
        ("Q".parse::<char>().unwrap(), 12),
        ("J".parse::<char>().unwrap(), 11),
        ("T".parse::<char>().unwrap(), 10),
        ("9".parse::<char>().unwrap(), 9),
        ("8".parse::<char>().unwrap(), 8),
        ("7".parse::<char>().unwrap(), 7),
        ("6".parse::<char>().unwrap(), 6),
        ("5".parse::<char>().unwrap(), 5),
        ("4".parse::<char>().unwrap(), 4),
        ("3".parse::<char>().unwrap(), 3),
        ("2".parse::<char>().unwrap(), 2),
    ]);
    // Parse input data.
    contents.trim().lines().into_iter().for_each(|line| {
        Regex::new(r"((\d+|\w+){5}) (\d+)")
            .unwrap()
            .captures_iter(line)
            .for_each(|hand_value| {
                let mut hand_cards: Vec<(char, u8)> = Vec::new();
                hand_value
                    .get(1)
                    .unwrap()
                    .as_str()
                    .to_owned()
                    .chars()
                    .into_iter()
                    .for_each(|card| {
                        if let Some(item) =
                            hand_cards.iter_mut().find(|&&mut (value, _)| value == card)
                        {
                            item.1 += 1;
                        } else {
                            hand_cards.push((card, 1));
                        }
                    });

                hands.push((
                    hand_value.get(1).unwrap().as_str().to_owned(),
                    hand_value
                        .get(3)
                        .unwrap()
                        .as_str()
                        .to_owned()
                        .parse::<u16>()
                        .unwrap(),
                    1,
                    hand_cards,
                ));
            });
    });

    // Calculate hand strength
    hands.iter_mut().for_each(|hand| {
        hand.3.iter().for_each(|hand_card| match hand_card.1 {
            // Repetition values are meant to separate having two doubles be worth the same as a triple, and so on.
            2 => hand.2 += 1,
            3 => hand.2 += 3,
            4 => hand.2 += 5,
            5 => hand.2 += 6,
            _ => (),
        });
    });

    hands.sort_by(|(hand_a, _, strength_a, _), (hand_b, _, strength_b, _)| {
        if strength_a != strength_b {
            strength_a.cmp(&strength_b)
        } else {
            for (card_a, card_b) in hand_a
                .chars()
                .into_iter()
                .to_owned()
                .zip(hand_b.chars().into_iter().to_owned())
            {
                if card_a != card_b {
                    return card_values
                        .get(&card_a)
                        .unwrap()
                        .cmp(card_values.get(&card_b).unwrap());
                }
            }
            strength_a.cmp(&strength_b)
        }
    });

    let mut value: u32 = 0;
    let mut position: u16 = 1;
    hands.iter().for_each(|hand| {
        value += hand.1 as u32 * position as u32;
        position += 1;
    });

    println!("{}", value);
}
