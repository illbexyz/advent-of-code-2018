use std::fs;
use std::io::{Result};
use std::collections::HashSet;

fn read_file(filepath: &str) -> Result<Vec<i32>> {
    let data = fs::read_to_string(filepath).expect("File not found");
    let data_as_list: Vec<i32> = data.split_whitespace().map(|e| {
        e.parse::<i32>().expect("Can't convert to i32")
    }).collect();

    Ok(data_as_list)
}

fn main() {
    let frequencies = read_file("input.txt").unwrap();
    let sum = frequencies.iter().fold(0, |x, y| { x + y } );

    let mut twice_freq = 0;
    let mut hset = HashSet::new();
    hset.insert(0);

    for x in frequencies.iter().cycle() {
        twice_freq += x;
        if hset.contains(&twice_freq) {
            break
        } else {
            hset.insert(twice_freq);
        }
    };

    println!("Part one: {}", sum);
    println!("Part two: {}", twice_freq)
}
