extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::io::Result;

#[derive(Debug)]
struct FabricSlice {
    id: i32,
    left: i32,
    top: i32,
    width: i32,
    height: i32,
}

fn read_file(filepath: &str) -> Result<Vec<FabricSlice>> {
    let data = fs::read_to_string(filepath).expect("File not found");
    let re = Regex::new(r"(?m)^#([0-9]+) @ ([0-9]+),([0-9]+): ([0-9]+)x([0-9]+)").unwrap();
    let slices: Vec<FabricSlice> = re
        .captures_iter(&data)
        .map(|cap| FabricSlice {
            id: cap[1].parse::<i32>().expect("Can't convert to i32"),
            left: cap[2].parse::<i32>().expect("Can't convert to i32"),
            top: cap[3].parse::<i32>().expect("Can't convert to i32"),
            width: cap[4].parse::<i32>().expect("Can't convert to i32"),
            height: cap[5].parse::<i32>().expect("Can't convert to i32"),
        }).collect();

    Ok(slices)
}

fn main() {
    let slices = read_file("input.txt").unwrap();
    let mut hmap = HashMap::new();

    for s in slices.iter() {
        for i in s.left..s.left + s.width {
            for j in s.top..s.top + s.height {
                let prev_value: i32 = match hmap.get(&(i, j)) {
                    Some(v) => *v,
                    None => 0,
                };
                hmap.insert((i, j), prev_value + 1);
            }
        }
    }

    let overlap_count = hmap
        .values()
        .fold(0, |acc, v| if *v > 1 { acc + 1 } else { acc });

    println!("Part one: {}", overlap_count);

    let mut intact_slice = None;
    for s in slices.iter() {
        let mut is_intact = true;
        for i in s.left..s.left + s.width {
            for j in s.top..s.top + s.height {
                let prev_value: i32 = match hmap.get(&(i, j)) {
                    Some(v) => *v,
                    None => 0,
                };
                if prev_value != 1 {
                    is_intact = false;
                }
            }
        }
        if is_intact {
            intact_slice = Some(s);
            break;
        }
    }

    match intact_slice {
        Some(s) => println!("Part two: {}", s.id),
        None => println!("Part two: not found"),
    }
}
