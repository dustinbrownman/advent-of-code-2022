use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Result};
use std::path::Path;
// use std::str::FromStr;
use itertools::Itertools;

// [T]             [P]     [J]
// [F]     [S]     [T]     [R]     [B]
// [V]     [M] [H] [S]     [F]     [R]
// [Z]     [P] [Q] [B]     [S] [W] [P]
// [C]     [Q] [R] [D] [Z] [N] [H] [Q]
// [W] [B] [T] [F] [L] [T] [M] [F] [T]
// [S] [R] [Z] [V] [G] [R] [Q] [N] [Z]
// [Q] [Q] [B] [D] [J] [W] [H] [R] [J]
//  1   2   3   4   5   6   7   8   9

fn generate_stacks() -> [Vec<&'static str>; 9] {
    [
        vec!["Q", "S", "W", "C", "Z", "V", "F", "T"],
        vec!["Q", "R", "B"],
        vec!["B", "Z", "T", "Q", "P", "M", "S"],
        vec!["D", "V", "F", "R", "Q", "H"],
        vec!["J", "G", "L", "D", "B", "S", "T", "P"],
        vec!["W", "R", "T", "Z"],
        vec!["H", "Q", "M", "N", "S", "F", "R", "J"],
        vec!["R", "N", "F", "H", "W"],
        vec!["J", "Z", "T", "Q", "P", "R", "B"]
    ]
}

// create struct CargoShip that has a stacks property
struct CargoShip {
    stacks: [Vec<&'static str>; 9],
}

// implement a move function on CargoShip that takes a tuple of i8
impl CargoShip {
    fn move_crate(&mut self, (amount, from, to): (i8, i8, i8)) {
        for _ in 0..amount {
            let this_crate = self.stacks[(from - 1) as usize].pop().unwrap();
            self.stacks[(to - 1) as usize].push(this_crate);
        }
    }
}

fn part_one_solution(lines: Lines<BufReader<File>>) {
    let mut ship = CargoShip { stacks: generate_stacks() };

    for line in lines {
        if let Ok(line) = line {
            let instructions = parse_instructions(line);
            ship.move_crate(instructions);
        } else {
            println!("Error reading line")
        }
    }

    let mut top_crates = String::new();
    for stack in ship.stacks.iter() {
        top_crates.push_str(stack.last().unwrap());
    }
    println!("Top crates: {}", top_crates);
}

fn part_two_solution(_lines: Lines<BufReader<File>>) {

}

fn parse_instructions(line: String) -> (i8, i8, i8) {
    let pattern = regex::Regex::new(r"\d+").unwrap();

    // extract all the matches and convert them to integers
    if let Some(matches) = pattern
        .find_iter(line.as_str())
        .take(3)
        .map(|m| m.as_str().parse::<i8>().unwrap())
        .collect_tuple::<(i8, i8, i8)>() {
            matches
        } else {
            println!("No matches found");
            (0, 0, 0)
        }
}

fn main() {
    let mut args = std::env::args().collect::<Vec<String>>();

    args.remove(0);

    if let Ok(lines) = read_lines("./input.txt") {
        match args.pop() {
            Some(solution) => match solution.as_str() {
                "1" => part_one_solution(lines),
                "2" => part_two_solution(lines),
                _ => println!("Unknown solution"),
            },
            None => part_one_solution(lines),
        }
    } else {
        println!("There was an error reading the input file")
    }
}

// From https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
