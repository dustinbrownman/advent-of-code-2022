use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Result};
use std::path::Path;

fn main() {
    let mut args = std::env::args().collect::<Vec<String>>();

    args.remove(0);

    if let Ok(lines) = read_lines("./input.txt") {
        match args.pop() {
            Some(solution) => match solution.as_str() {
                "1" => part_one_solution(lines),
                // "2" => part_two_solution(),
                _ => println!("Unknown solution"),
            },
            None => part_one_solution(lines),
        }
    } else {
        println!("There was an error reading the input file")
    }
}

fn part_one_solution(lines: Lines<BufReader<File>>) {
    let mut count = 0;

    for line in lines {
        if let Ok(line) = line {
            let (first, second) = parse_ranges(&line);

            if within_range(first, second) || within_range(second, first) {
                count += 1;
            }
        } else {
            println!("Error reading line")
        }
    }

    println!("Pairs fully containing another: {}", count);
}

fn parse_ranges(r: &str) -> ((u32, u32), (u32, u32)) {
    if let Some(ranges) = r.split_once(',') {
        if let (Some(first), Some(second)) = (ranges.0.split_once('-'), ranges.1.split_once('-')) {
            (
                (first.0.parse::<u32>().unwrap(), first.1.parse::<u32>().unwrap()),
                (second.0.parse::<u32>().unwrap(), second.1.parse::<u32>().unwrap()),
            )
        } else {
            println!("There was an error creating tuples");
            ((0, 0), (0, 0))
        }
    } else {
        println!("Problem parsing initial string");
        ((0, 0), (0, 0))
    }
}

fn within_range(outer: (u32, u32), inner: (u32, u32)) -> bool {
    outer.0 <= inner.0 && inner.1 <= outer.1
}

// From https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
