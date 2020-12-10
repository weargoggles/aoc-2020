use util::data_file;
extern crate itertools;
use itertools::Itertools;
use std::io::{BufReader, BufRead};

extern crate anyhow;

fn main() -> anyhow::Result<()> {
    let ids: Vec<i32> = BufReader::new(data_file("boarding-passes.txt")?)
        .lines()
        .map(|l| l.unwrap())
        .map(pass_to_id)
        .collect();

    println!("part 1: {:#?}", ids.iter().max());
    let sorted: Vec<&i32> = ids.iter().sorted().collect();

    for chunk in sorted.windows(2) {
        if chunk[1] - chunk[0] > 1 {
            println!("part 2: {:#?}", chunk);
        }
    }

    Ok(())
}

fn pass_to_id(pass: String) -> i32 {
    let pass_val = pass.chars().fold(0, |acc, item| {
        acc << 1 | match item {
            'F' => 0,
            'B' => 1,
            'L' => 0,
            'R' => 1,
            _ => panic!("unexpected item in the bagging area"),
        }
    });
    pass_val
}


#[cfg(test)]
mod tests {
    use crate::pass_to_id;

    #[test]
    fn test_pass_to_id() {
        assert_eq!(pass_to_id("FBFBBFFRLR".to_string()), 357);
        assert_eq!(pass_to_id("BFFFBBFRRR".to_string()), 567);
        assert_eq!(pass_to_id("FFFBBBFRRR".to_string()), 119);
        assert_eq!(pass_to_id("BBFFBBFRLL".to_string()), 820);
    }
}
