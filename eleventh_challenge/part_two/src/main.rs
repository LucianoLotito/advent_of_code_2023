use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("../input.txt");
    let mut buf_reader = BufReader::new(file.unwrap());
    let mut contents = String::new();
    let _ = buf_reader.read_to_string(&mut contents);
    let lines: Vec<&str> = contents.lines().collect();
    let mut matrix: Vec<(u64, u64)> = vec![];

    for (ln_index, ln) in lines.iter().enumerate() {
        for (ch_index, ch) in ln.chars().enumerate() {
            if ch == '#' {
                matrix.push((ln_index as u64, ch_index as u64));
            }
        }
    }

    let mut expanded_matrix = matrix.clone();

    for mtx in 1..matrix.len() {
        if matrix[mtx].0 as i32 - matrix[mtx - 1].0 as i32 > 1 {
            let loop_ln = matrix[mtx].0 - matrix[mtx - 1].0;
            for _ in 1..loop_ln {
                for xpnd_ln in mtx..matrix.len() {
                    expanded_matrix[xpnd_ln].0 += 999_999;
                }
            }
        }
    }

    matrix.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    expanded_matrix.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    for mtx in 1..matrix.len() {
        if matrix[mtx].1 as i32 - matrix[mtx - 1].1 as i32 > 1 {
            let col_ln = matrix[mtx].1 - matrix[mtx - 1].1;
            for _ in 1..col_ln {
                for xpnd_col in mtx..matrix.len() {
                    expanded_matrix[xpnd_col].1 += 999_999;
                }
            }
        }
    }

    let mut total_steps: u64 = 0;
    for gal in 0..expanded_matrix.len() {
        for element in gal + 1..expanded_matrix.len() {
            total_steps += expanded_matrix[element].0.abs_diff(expanded_matrix[gal].0)
                + expanded_matrix[element].1.abs_diff(expanded_matrix[gal].1);
        }
    }
    println!("{}", total_steps);

    expanded_matrix.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
}
