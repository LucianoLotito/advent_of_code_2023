use std::fs::File;
use std::io::{self, BufRead};
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("../input.txt");
    let mut buf_reader = BufReader::new(file.unwrap());
    let mut contents = String::new();
    let _ = buf_reader.read_to_string(&mut contents);
    let lines: Vec<&str> = contents.lines().collect();
    let mut matrix: Vec<Vec<char>> = vec![];

    let mut clean_lines: Vec<u32> = vec![];
    let mut clean_cols: Vec<u32> = vec![];
    let mut col_galaxy = false;
    let mut line_galaxy = false;

    lines
        .iter()
        .map(|line| {
            let mut ln: Vec<char> = vec![];
            line.chars()
                .map(|ch| {
                    ln.push(ch.clone());
                })
                .count();
            matrix.push(ln.clone());
        })
        .count();

    for line in 0..lines.len() {
        line_galaxy = false;
        let mut ln: Vec<char> = vec![];
        for ch in 0..lines[line].len() {
            ln.push(lines[line].chars().nth(ch).unwrap().clone());
            if lines[line].chars().nth(ch).unwrap() != '#' {
                if clean_cols.contains(&(ch as u32)) {
                    continue;
                }
                col_galaxy = false;
                for sec_line in 0..lines.len() {
                    if lines[sec_line].chars().nth(ch).unwrap() == '#' {
                        col_galaxy = true;
                    }
                }
                if !col_galaxy {
                    clean_cols.push(ch as u32);
                }
            } else {
                line_galaxy = true;
                break;
            }
        }
        if !line_galaxy {
            clean_lines.push(line.clone() as u32);
        }
    }

    println!("{:?}", clean_cols);

    clean_cols
        .iter()
        .map(|f| {
            for line in 0..matrix.len() {
                matrix[line].insert(*f as usize, '.');
            }
        })
        .count();

    let new_clean_line: Vec<char> = matrix[0].iter().map(|f| '.').collect();

    clean_lines.reverse();

    clean_lines
        .iter()
        .map(|l| {
            matrix.insert(*l as usize, new_clean_line.clone());
        })
        .count();

    let mut new_matrix_galaxy_pos: Vec<(u32, u32)> = vec![];

    for line in 0..matrix.len() {
        for ch in 0..matrix[line].len() {
            if matrix[line][ch] == '#' {
                new_matrix_galaxy_pos.push((ch.clone() as u32, line.clone() as u32));
            }
        }
    }

    let mut total_steps: u32 = 0;
    for galaxy in 0..new_matrix_galaxy_pos.len() {
        for element in galaxy + 1..new_matrix_galaxy_pos.len() {
            total_steps += new_matrix_galaxy_pos[element]
                .0
                .abs_diff(new_matrix_galaxy_pos[galaxy].0)
                + new_matrix_galaxy_pos[element]
                    .1
                    .abs_diff(new_matrix_galaxy_pos[galaxy].1);
        }
    }
    println!("{}", total_steps);
}
