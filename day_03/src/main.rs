use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Result};
use std::path::Path;
use std::collections::HashSet;

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

fn part_one_solution() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut total_priority: u32 = 0;
        for line in lines {
            if let Ok(line) = line {
                let pack = string_to_priorities(&line);
                let sum = pack.chunks(pack.len() / 2).map(|chunk| -> HashSet<&u32> {
                    HashSet::from_iter(chunk.iter())
                })
                    .reduce(|a, b| a.intersection(&b).copied().collect())
                    .into_iter()
                    .flatten()
                    .sum::<u32>();

                total_priority += sum;
            } else {
                println!("Error reading line");
            }
        }

        println!("Total priority: {}", total_priority);
    }
}

fn string_to_priorities(input: &str) -> Vec<u32> {
    input.chars().map(|c| {
        let ascii_value = c as u32;

        match ascii_value {
            65..=90 => ascii_value - 64 + 26,
            97..=122 => ascii_value - 96,
            _ => 0,
        }
    }).collect()
}

// From https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

