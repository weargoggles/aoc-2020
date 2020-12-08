use util::data_file;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let trees = BufReader::new(data_file("trees.txt")?)
        .lines()
        .map(|l| l.unwrap())
        .zip((0..).step_by(3))
        .filter(|(a, b)| {
            a.as_bytes()[b % a.len()] == "#".as_bytes()[0]
        })
        .count();
    println!("trees: {}", trees);
    Ok(())
}
