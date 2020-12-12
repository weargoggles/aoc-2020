extern crate anyhow;

use util::data_file;
use std::io::{BufReader, Read, BufRead};
use std::collections::HashSet;

fn main() -> anyhow::Result<()> {
    let groups = {
        let mut groups = Vec::<Vec<HashSet<char>>>::new();
        let mut forms = Vec::<HashSet<char>>::new();
        for line in BufReader::new(data_file("customs-forms.txt")?)
            .lines()
            .map(|l| l.unwrap()) {
            forms = if line == "" {
                groups.push(forms);
                Vec::new()
            } else {
                forms.push(line.chars().collect());
                forms
            }
        }
        groups.push(forms);
        groups
    };

    // part 1 - sum of the sizes of (the union of all forms in each group)
    let part1 = groups
        .iter()
        .map(|group| {
            group.iter()
                .fold(HashSet::new(), |a, form| {
                    a.union(form).map(|c| c.clone()).collect()
                }).len()
        })
        .sum::<usize>();
    println!("part1: {:#?}", part1);

    // part 2 - the sum of the sizes of (the intersection of all forms in each group)
    let questions = ('a'..='z').collect::<HashSet<char>>();
    let part2 = groups
        .iter()
        .map(|group| {
            group.iter().fold(questions.clone(), |a, form| {
                a.intersection(form).map(|c| c.clone()).collect()
            }).len()
        })
        .sum::<usize>();
    println!("part2: {:#?}", part2);
    Ok(())
}
