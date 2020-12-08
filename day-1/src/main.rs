use std::fs;
use std::io::prelude::*;
use util::application_root_dir;
use std::io::BufReader;
use std::str::FromStr;

fn main() -> std::io::Result<()>{
    let app_root = application_root_dir()?;
    println!("application root: {:#?}", app_root);
    let expenses_path = app_root.join("data").join("expenses.txt");
    let file = fs::File::open(expenses_path)?;
    let values: Vec<i64> = BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .map(|s| i64::from_str(&s))
        .map(Result::unwrap)
        .collect();
    let deduped = {
        let mut cloned = values.clone();
        cloned.dedup();
        cloned
    };

    assert_eq!(deduped, values);

    for i in values.iter() {
        for j in values.iter() {
            if i + j == 2020 {
                println!("part 1: {}", i * j);
            }
        }
    }

    for i in values.iter() {
        for j in values.iter() {
            for k in values.iter() {
                if i + j + k == 2020 {
                    println!("part 2: {}", i * j * k);
                }
            }
        }
    }
    Ok(())
}
