use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Result};
use std::path::Path;

fn main() {
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
