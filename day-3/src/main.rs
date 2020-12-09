use util::data_file;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, Copy, Clone)]
struct Slope {
    x: usize,
    y: usize,
}

fn main() -> std::io::Result<()> {
    let trees = BufReader::new(data_file("trees.txt")?)
        .lines()
        .map(|l| l.unwrap())
        .zip((0..).step_by(3))
        .filter(|(a, b)| {
            a.as_bytes()[b % a.len()] == "#".as_bytes()[0]
        })
        .count();
    println!("part1: {}", trees);

    let slopes = vec![
        Slope { x: 1, y: 1 },
        Slope { x: 3, y: 1 },
        Slope { x: 5, y: 1 },
        Slope { x: 7, y: 1 },
        Slope { x: 1, y: 2 },
    ];
    let a_tree = "#".as_bytes()[0];
    let trees_hit: Vec<usize> = slopes.iter().map(|_| 0).collect(); // hit counter for each slope
    let part2 = BufReader::new(data_file("trees.txt")?)
        .lines()
        .map(|l| l.unwrap())
        .zip((0..)) // track the y-distance
        .fold(trees_hit, |counters, (l, y)| { // for each row in the map
            counters.iter().zip(slopes.clone())
                .map(|(counter, s)| {
                    if y % s.y == 0 && l.as_bytes()[(y * s.x / s.y) % l.len()] == a_tree {
                        counter + 1
                    } else {
                        *counter
                    }
                }).collect()
        });
    println!("part2: {:#?}", part2);
    let product: usize = part2.iter().product();
    println!("product: {}", product);
    Ok(())
}
