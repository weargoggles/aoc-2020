use std::io::prelude::*;
use util::data_file;
use std::io::BufReader;
use std::str::FromStr;
use itertools::Itertools;

fn main() -> std::io::Result<()>{
    let values = read_values()?;

    for pair in values.clone().into_iter().combinations(2) {
        if pair[0] + pair[1] == 2020 {
            println!("pair: {}", pair[0] * pair[1]);
            break
        }
    }

    for triple in values.clone().into_iter().combinations(3) {
        if triple[0] + triple[1] + triple[2] == 2020 {
            println!("triple: {}", triple[0] * triple[1] * triple[2]);
            break
        }
    }

    Ok(())
}

fn read_values() -> std::io::Result<Vec<i64>> {
    let file = data_file("expenses.txt")?;
    let values: Vec<i64> = BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .map(|s| i64::from_str(&s))
        .map(Result::unwrap)
        .collect();
    Ok(values)
}
