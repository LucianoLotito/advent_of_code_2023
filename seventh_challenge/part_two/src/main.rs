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
        ("J".parse::<char>().unwrap(), 1),
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
        // Use regex to find and group the hands and the bets.
        Regex::new(r"((\d+|\w+){5}) (\d+)")
            .unwrap()
            .captures_iter(line)
            .for_each(|hand_value| {
                let mut hand_cards: Vec<(/*Card value*/ char, /*Card instances*/ u8)> = Vec::new();
                // Iterate the hands and count each of them into the hand_cards tuple vector.
                hand_value
                    .get(1)
                    .unwrap()
                    .as_str()
                    .to_owned()
                    .chars()
                    .into_iter()
                    .for_each(|card| {
                        // If the card is not present in the tuple, insert it, else, count one more instance of it.
                        if let Some(item) =
                            hand_cards.iter_mut().find(|&&mut (value, _)| value == card)
                        {
                            item.1 += 1;
                        } else {
                            hand_cards.push((card, 1));
                        }
                    });

                let mut hand_card_sorted = hand_cards.clone();
                // Sort the hand cards by its power in the card_value vector tuple, least powerful first.
                hand_card_sorted.sort_by(|(card_a, _), (card_b, _)| {
                    card_values
                        .get(&card_a)
                        .unwrap()
                        .cmp(card_values.get(&card_b).unwrap())
                });

                // Find out the number of jokers present in the hand tuple vector.
                let number_of_jokers = match hand_cards
                    .iter()
                    .find(|&&(card, _)| card == "J".parse::<char>().unwrap())
                {
                    None => 0,
                    Some(t) => t.1.to_owned(),
                };

                // Remove the Joker tuple from the hand vector.
                hand_cards.retain(|(card, _)| card != &"J".parse::<char>().unwrap());
                // Sort the hand by the number of instances of each card, least instances first.
                hand_cards.sort_by(|(_, value_a), (_, value_b)| value_a.cmp(value_b));
                match hand_cards.last_mut() {
                    // In case the tuple is empty, means that the hand was made out of all Jokers, which means the
                    // new hand should be composed of all As
                    None => hand_cards.push(("A".parse::<char>().unwrap(), 5 as u8)),
                    // Else, sum the number of jokers to the most repeated card in the hand, which will be the most convenient
                    // transformation for the Joker card.
                    Some(t) => t.1 = t.1 + number_of_jokers,
                };

                // Push the newly composed and formated hand into the hand cards structure vector.
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

    // Sort the hands by hand and/or strength
    hands.sort_by(|(hand_a, _, strength_a, _), (hand_b, _, strength_b, _)| {
        // If both hands have different strengths, then order them by it.
        if strength_a != strength_b {
            strength_a.cmp(&strength_b)
        }
        // Else, iterate each hand card and order them by the value of the first differing card, least powerful first.
        else {
            for (card_a, card_b) in hand_a
                .chars()
                .into_iter()
                .to_owned()
                .zip(hand_b.chars().into_iter().to_owned())
            {
                if card_a != card_b {
                    // Once the difference is found, return the ordering and break the loop.
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
    // Iterate each hand, multiply its bet by its position in the vector, starting from one and sum the result to the accumulator.
    hands.iter().for_each(|hand| {
        value += hand.1 as u32 * position as u32;
        position += 1;
    });

    println!("{}", value);
}
