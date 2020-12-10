use util::data_file;
extern crate itertools;
use itertools::Itertools;
use std::io::{BufReader, BufRead};
extern crate anyhow;

fn main() -> anyhow::Result<()> {
    let max_id= BufReader::new(data_file("boarding-passes.txt")?)
        .lines()
        .map(|l| pass_to_id(l.unwrap().as_str()))
        .max();
    println!("part 1: {:#?}", max_id);

    let sorted: Vec<i32> = BufReader::new(data_file("boarding-passes.txt")?)
        .lines()
        .map(|l| pass_to_id(l.unwrap().as_str()))
        .sorted().collect();

    for chunk in sorted.windows(2) {
        if chunk[1] - chunk[0] > 1 {
            println!("part 2: {:#?}", chunk);
        }
    }

    Ok(())
}

fn pass_to_id(pass: &str) -> i32 {
    let pass_val = pass.chars().fold(0, |acc, item| {
        let acc = acc << 1;
        let res = acc | match item {
            'F' => 0,
            'B' => 1,
            'L' => 0,
            'R' => 1,
            _ => panic!("unexpected item in the bagging area"),
        };
        res
    });
    pass_val
}


#[cfg(test)]
mod tests {
    use crate::pass_to_id;

    #[test]
    fn test_pass_to_id() {
        assert_eq!(pass_to_id("FBFBBFFRLR"), 357);
        assert_eq!(pass_to_id("BFFFBBFRRR"), 567);
        assert_eq!(pass_to_id("FFFBBBFRRR"), 119);
        assert_eq!(pass_to_id("BBFFBBFRLL"), 820);
    }
}
