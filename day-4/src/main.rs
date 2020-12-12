use std::io::{BufReader, BufRead};
use util::data_file;
extern crate regex;
use regex::Regex;
extern crate anyhow;
use std::collections::HashSet;

fn main() -> anyhow::Result<()> {
    let required: HashSet<&str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"].into_iter().collect();
    let passports: Vec<Vec<(String, String)>> = BufReader::new(data_file("passports.txt")?)
        .lines()
        .map(|l| l.unwrap())
        .fold(vec![Vec::new()], |passports, item| {
            let mut p = passports.clone();
            if item == "" {
                p.push(Vec::new());
                p
            } else {
                for field in item.split(" ") {
                    let parts: Vec<&str> = field.split(":").collect();
                    p.last_mut().unwrap().push((String::from(parts[0]), String::from(parts[1])))
                }
                p
            }
        });

    let valid_part1: Vec<&Vec<(String, String)>> = passports.iter().filter(|passport| {
        let fields: HashSet<&str> = passport.into_iter().map(|field| {
            let (key, value) = field;
            key.as_str()
        }).collect();
        required.difference(&fields).count() == 0
    }).collect();
    println!("valid passports for part1: {:#?}", valid_part1.len());

    let valid_hair_colours = Regex::new(r"^#[0-9a-f]{6}$")?;
    let valid_eye_colours = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$")?;
    let valid_passport_id = Regex::new(r"^[0-9]{9}$")?;
    let year_between: fn(&str, &str) -> (fn(&str) -> bool) = |min, max| {
        |year| year.len() == 4 && min <= year && max >= year
    };
    let valid_part2: Vec<&Vec<(String, String)>> = valid_part1.into_iter().filter(|passport| {
        passport.into_iter().all(|field| {
            let (k, v) = field;
            match (k.as_str(), v.as_str()) {
                ("byr", year) => year_between("1920", "2002")(year),
                ("iyr", year) => year.len() == 4 && "2010" <= year && "2020" >= year,
                ("eyr", year) => year.len() == 4 && "2020" <= year && "2030" >= year,
                ("hgt", height) => {
                    match height.split_at(height.len() - 2) {
                        (h, "cm") => h.len() == 3 && "150" <= h && "193" >= h,
                        (h, "in") => h.len() == 2 && "59" <= h && "76" >= h,
                        (_, _) => false,
                    }
                },
                ("hcl", hair_color) => valid_hair_colours.is_match(hair_color),
                ("ecl", eye_color) => valid_eye_colours.is_match(eye_color),
                ("pid", passport_id) => valid_passport_id.is_match(passport_id),
                (_, _) => true,
            }
        })
    }).collect();
    println!("valid passports for part2: {:#?}", valid_part2.len());

    Ok(())
}
