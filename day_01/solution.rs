use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Result};
use std::path::Path;

fn main() {
    let mut args = std::env::args().collect::<Vec<String>>();

    args.remove(0);

    match args.pop() {
        Some(solution) => match solution.as_str() {
            "1" => part_one_solution(),
            "2" => part_two_solution(),
            _ => println!("Unknown solution"),
        },
        None => part_one_solution(),
    }
}

fn part_one_solution() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut current_calorie_count: u32 = 0;
        let mut elf_packs = Vec::new();

        for line in lines {
            if let Ok(line) = line {
                match parse_line(line) {
                    Some(item_in_calories) => current_calorie_count += item_in_calories,
                    None => {
                        elf_packs.push(current_calorie_count);
                        current_calorie_count = 0;
                    }
                }
            }
        }

        let biggest_elf_pack = elf_packs.iter().max().unwrap();
        println!("The biggest elf pack has {} calories", biggest_elf_pack);
    }
}

fn part_two_solution() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut current_calorie_count: u32 = 0;
        let mut largest_elf_packs = vec![0; 3];

        for line in lines {
            if let Ok(line) = line {
                match parse_line(line) {
                    Some(item_in_calories) => current_calorie_count += item_in_calories,
                    None => {
                        // Look for a pack among the largest that is less than the current one
                        match largest_elf_packs.iter().position(|&pack| pack < current_calorie_count) {
                            Some(index) => {
                                largest_elf_packs.insert(index, current_calorie_count);
                                largest_elf_packs.pop();
                            }
                            None => (),
                        }
                        current_calorie_count = 0;
                    }
                }
            }
        }

        let sum_of_largest_elf_packs: u32 = largest_elf_packs.iter().sum();
        println!("The sum of the largest elf packs is {}", sum_of_largest_elf_packs);
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

fn parse_line(line: String) -> Option<u32> {
    let calorie_count = line.parse::<u32>();
    match calorie_count {
        Ok(calorie_count) => Some(calorie_count),
        Err(_) => None,
    }
}
