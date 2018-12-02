use std::collections::HashMap;
use std::fs;
use std::io::Result;

fn read_file(filepath: &str) -> Result<Vec<String>> {
    let data = fs::read_to_string(filepath).expect("File not found");
    let data_as_list: Vec<String> = data.split_whitespace().map(|e| e.to_string()).collect();

    Ok(data_as_list)
}

fn main() {
    let ids = read_file("input.txt").unwrap();

    let (twos, threes): (Vec<(bool)>, Vec<bool>) = ids
        .iter()
        .map(|id| {
            let mut hmap = HashMap::new();
            let mut has_two = false;
            let mut has_three = false;

            for c in id.chars() {
                let value = match hmap.get(&c) {
                    Some(v) => v + 1,
                    None => 1,
                };
                hmap.insert(c, value);
            }

            for (_key, val) in hmap.iter() {
                if *val == 2 {
                    has_two = true;
                } else if *val == 3 {
                    has_three = true;
                }
            }

            (has_two, has_three)
        }).unzip();

    let count_two = twos.iter().fold(0, |x, y| if *y { x + 1 } else { x });
    let count_three = threes.iter().fold(0, |x, y| if *y { x + 1 } else { x });

    println!("Part one: {}", count_two * count_three);

    let mut pair = (vec![], vec![]);
    for (i, id1) in ids.iter().enumerate() {
        for (j, id2) in ids.iter().enumerate() {
            if i < j {
                let chars1: Vec<char> = id1.chars().collect();
                let chars2: Vec<char> = id2.chars().collect();
                let mut diffs = 0;
                for i in 0..id1.len() {
                    if chars1.get(i).unwrap() != chars2.get(i).unwrap() {
                        diffs += 1;
                    }
                }
                if diffs == 1 {
                    pair = (id1.chars().collect(), id2.chars().collect());
                    break;
                }
            }
        }
    }

    let (a, b) = pair;

    let code = a.iter().zip(b.iter()).fold(String::new(), |acc, (a, b)| {
        if a != b {
            acc
        } else {
            format!("{}{}", acc, b)
        }
    });

    println!("Part two: {}", code)
}
