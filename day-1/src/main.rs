use std::fs;
use std::io::prelude::*;
use util::data_file;
use std::io::BufReader;
use std::str::FromStr;
use std::iter::Product;
use itertools::Itertools;

fn main() -> std::io::Result<()>{
    let values = read_values()?;

    let pair: Vec<i64> = values.clone().into_iter().combinations(2)
        .filter(|v| v.iter().sum::<i64>() == 2020)
        .map(|v| v.iter().product::<i64>())
        .collect();

    let triple: Vec<i64> = values.clone().into_iter().combinations(3)
        .filter(|v| v.iter().sum::<i64>() == 2020)
        .map(|v| v.iter().product::<i64>())
        .collect();

    println!("pair: {:#?}, triple: {:#?}", pair, triple);
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
