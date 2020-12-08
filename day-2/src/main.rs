use util;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::Matches;
use scan_fmt::parse::scan;
use std::fs;

#[macro_use]
extern crate scan_fmt;

fn main() -> std::io::Result<()> {
    println!("part1: {:#?}", validate_passwords(part1));
    println!("part2: {:#?}", validate_passwords(part2));
    Ok(())
}

fn validate_passwords(predicate: fn(&(i32, i32, String, String)) -> bool) -> std::io::Result<usize> {
    let data = util::data_file("passwords.txt")?;
    Ok(BufReader::new(data).lines()
        .map(Result::unwrap)
        .map(read_password)
        .filter(predicate)
        .count())
}

fn part1((min, max, character, password): &(i32, i32, String, String)) -> bool {
    let matches: i32 = password.matches(character).count() as i32;
    (*min..*max + 1).contains(&matches)
}

fn character_equal(location: &i32, character: &String, password: &String) -> bool {
    match password.chars().nth((*location as usize) - 1) {
        Some(c) => {
            &c.to_string() == character
        }
        None => false
    }
}

fn part2((min, max, character, password): &(i32, i32, String, String)) -> bool {
    let a = character_equal(min, character, password);
    let b = character_equal(max, character, password);
    let e = a ^ b;
    println!("{}, {}, {}, {}. {}, {}, {}", min, max, character, password, a, b, e);
    e
}

fn read_password(line: String) -> (i32, i32, String, String) {
    scan_fmt!(&line, "{d}-{d} {}: {}", i32, i32, String, String).unwrap()
}
