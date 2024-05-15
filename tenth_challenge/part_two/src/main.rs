use gcd::Gcd;
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug)]
struct Possibilities {
    top: Option<usize>,
    bottom: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
}
struct Positioning {
    left: String,
    right: String,
    top: String,
    bottom: String,
}

#[derive(Debug, Clone, PartialEq)]
struct Position {
    line: usize,
    char: usize,
}
fn main() {
    let file = File::open("../input.txt");
    let mut buf_reader = BufReader::new(file.unwrap());
    let mut contents = String::new();
    let _ = buf_reader.read_to_string(&mut contents);
    let lines: Vec<&str> = contents.lines().collect();

    let initial_valid_positions = Positioning {
        left: "LF-".to_string(),
        right: "J7-".to_string(),
        top: "F7|".to_string(),
        bottom: "LJ|".to_string(),
    };

    let mut entry_point: Position = Position { line: 0, char: 0 };

    for line in 0..lines.len() {
        match lines[line].find('S') {
            Some(entry) => {
                entry_point = Position {
                    line: line.to_owned() as usize,
                    char: entry.to_owned() as usize,
                };
                break;
            }
            None => (),
        }
    }

    let mut current_pos = entry_point.clone();
    let mut previous_pos = entry_point.clone();

    let mut possibilities = Possibilities {
        left: None,
        right: None,
        top: None,
        bottom: None,
    };

    if current_pos.line as i8 - 1 >= 0 {
        if initial_valid_positions
            .top
            .find(
                lines[current_pos.line - 1]
                    .chars()
                    .nth(current_pos.char)
                    .unwrap()
                    .clone()
                    .to_string()
                    .as_str(),
            )
            .is_some()
        {
            current_pos.line = current_pos.line - 1;
        }
    }
    if current_pos.char + 1 <= lines[0].len() {
        if initial_valid_positions
            .right
            .find(
                lines[current_pos.line]
                    .chars()
                    .nth(current_pos.char + 1)
                    .unwrap()
                    .clone()
                    .to_string()
                    .as_str(),
            )
            .is_some()
        {
            current_pos.char = current_pos.char + 1;
        }
    }
    if current_pos.line + 1 <= lines.len() {
        if initial_valid_positions
            .bottom
            .find(
                lines[current_pos.line + 1]
                    .chars()
                    .nth(current_pos.char)
                    .unwrap()
                    .clone()
                    .to_string()
                    .as_str(),
            )
            .is_some()
        {
            current_pos.line = current_pos.line + 1;
        }
    }
    if current_pos.char as i8 - 1 >= 0 {
        if initial_valid_positions
            .left
            .find(
                lines[current_pos.line]
                    .chars()
                    .nth(current_pos.char - 1)
                    .unwrap()
                    .clone()
                    .to_string()
                    .as_str(),
            )
            .is_some()
        {
            current_pos.char = current_pos.char - 1;
        }
    }

    let valid_positions = Positioning {
        left: "7J-".to_string(),
        right: "FL-".to_string(),
        top: "JL|".to_string(),
        bottom: "F7|".to_string(),
    };

    let mut movements = 1;
    let mut points: Vec<(i16, i16)> = vec![(
        entry_point.char.clone() as i16,
        entry_point.line.clone() as i16,
    )];

    while (current_pos.line != entry_point.line) || (current_pos.char != entry_point.char) {
        let ourserlves = lines[current_pos.line]
            .chars()
            .nth(current_pos.char)
            .unwrap()
            .clone()
            .to_string();
        possibilities.top = None;
        possibilities.bottom = None;
        possibilities.left = None;
        possibilities.right = None;

        points.push((
            current_pos.char.clone() as i16,
            current_pos.line.clone() as i16,
        ));

        possibilities.top = valid_positions.top.find(ourserlves.clone().as_str());
        possibilities.right = valid_positions.right.find(ourserlves.clone().as_str());
        possibilities.bottom = valid_positions.bottom.find(ourserlves.clone().as_str());
        possibilities.left = valid_positions.left.find(ourserlves.clone().as_str());

        if possibilities.top.is_some() && (previous_pos.line != current_pos.line - 1) {
            previous_pos = current_pos.clone();
            current_pos.line = current_pos.line - 1;
        } else if possibilities.right.is_some() && previous_pos.char != current_pos.char + 1 {
            previous_pos = current_pos.clone();
            current_pos.char = current_pos.char + 1;
        } else if possibilities.bottom.is_some() && previous_pos.line != current_pos.line + 1 {
            previous_pos = current_pos.clone();
            current_pos.line = current_pos.line + 1;
        } else if possibilities.left.is_some() && previous_pos.char != current_pos.char - 1 {
            previous_pos = current_pos.clone();
            current_pos.char = current_pos.char - 1;
        }
        movements += 1;
    }

    println!("Furthest away: {}", movements / 2);

    let mut prev_point: (i16, i16);
    let mut current_point: (i16, i16);
    let mut next_point: (i16, i16);

    let mut vertexes: Vec<(i16, i16)> = vec![points.first().unwrap().clone()];

    for i in 1..points.len() - 1 {
        prev_point = points[i - 1];
        current_point = points[i];
        next_point = points[(i + 1) % (points.len() - 1)];

        if cross_product(prev_point, current_point, next_point) != 0 {
            vertexes.push(current_point);
        }
    }

    let area = polygon_area(vertexes.clone());

    /*
        Pick's theorem
        I = A - (B/2) +1
        A = area of a polygon
        B = number of lattice points on the boundary of the polygon
    */

    let lattice_points = polygon_lattice_points(vertexes.clone());

    println!(
        "Total points contained in the polygon: {}",
        area - (lattice_points / 2) + 1
    );
}

fn cross_product(o: (i16, i16), a: (i16, i16), b: (i16, i16)) -> i16 {
    ((a.0 - o.0) * (b.1 - o.1) - (a.1 - o.1) * (b.0 - o.0)).into()
}

fn polygon_area(vertexes: Vec<(i16, i16)>) -> u16 {
    let mut area: i16 = 0;

    let mut prev_vertex: (i16, i16) = vertexes.last().unwrap().to_owned();
    let mut current_vertex: (i16, i16) = vertexes.first().unwrap().to_owned();

    area += (prev_vertex.0 * current_vertex.1) - (prev_vertex.1 * current_vertex.0);

    for vertex in 1..vertexes.len() {
        prev_vertex = vertexes[vertex - 1];
        current_vertex = vertexes[vertex];

        area += (prev_vertex.0 * current_vertex.1) - (prev_vertex.1 * current_vertex.0);
    }

    u16::try_from(area / 2).unwrap()
}

fn polygon_lattice_points(vertexes: Vec<(i16, i16)>) -> u16 {
    let mut lattice_points: u16 = 0;

    let mut prev_vertex: (i16, i16) = vertexes.last().unwrap().to_owned();
    let mut current_vertex: (i16, i16) = vertexes.first().unwrap().to_owned();

    lattice_points += u16::try_from((prev_vertex.0 - current_vertex.0).abs())
        .unwrap()
        .gcd(u16::try_from((prev_vertex.1 - current_vertex.1).abs()).unwrap())
        + 1;

    for vertex in 1..vertexes.len() {
        prev_vertex = vertexes[vertex - 1];
        current_vertex = vertexes[vertex];
        lattice_points += u16::try_from((prev_vertex.0 - current_vertex.0).abs())
            .unwrap()
            .gcd(u16::try_from((prev_vertex.1 - current_vertex.1).abs()).unwrap())
            + 1;
    }

    lattice_points - vertexes.len() as u16
}
