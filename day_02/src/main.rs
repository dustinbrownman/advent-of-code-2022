use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Result};
use std::path::Path;

fn main() {
    let mut args = std::env::args().collect::<Vec<String>>();

    args.remove(0);

    match args.pop() {
        Some(solution) => match solution.as_str() {
            "1" => part_one_solution(),
            // "2" => part_two_solution(),
            _ => println!("Unknown solution"),
        },
        None => part_one_solution(),
    }
}

// A/X: rock
// B/Y: paper
// C/Z: scissors

fn part_one_solution() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut total_points: u32 = 0;

        for line in lines {
            if let Ok(line) = line {
                println!("{}", line);
                if let Some(round) = line.split_once(' ') {
                    total_points += points_for_round(round)
                }
            } else {
                println!("Error reading line");
            }
        }

        println!("Total points: {}", total_points);
    }
}

fn points_for_round((opponent, mine): (&str, &str)) -> u32 {
    let mut points: u32 = 0;

    let winning_combos = vec![("A", "Y"), ("B", "Z"), ("C", "X")];
    let drawing_combos = vec![("A", "X"), ("B", "Y"), ("C", "Z")];
    // let losing_combos = vec![("A", "Z"), ("B", "X"), ("C", "Y")];

    if winning_combos.contains(&(opponent, mine)) {
        points += 6;
    } else if drawing_combos.contains(&(opponent, mine)) {
        points += 3;
    }

    match mine {
        "X" => points += 1,
        "Y" => points += 2,
        "Z" => points += 3,
        _ => (),
    }

    points
}

// From https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
