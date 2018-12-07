use std::fs;

#[derive(Copy, Clone, Debug)]
pub enum Polarity {
    Positive,
    Negative,
}

#[derive(Copy, Clone, Debug)]
pub struct Unit {
    t: char,
    polarity: Polarity,
}

type Polymer = Vec<Unit>;

pub fn show_polymer(polymer: &Polymer) -> String {
    let mut s = String::new();
    for unit in polymer {
        s.push(unit.t);
    }
    s
}

pub fn read_data(filepath: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(filepath)
}

pub fn parse_data(data: String) -> Vec<Unit> {
    let polymer = data
        .chars()
        .map(|c| Unit {
            t: c,
            polarity: if c.is_lowercase() {
                Polarity::Negative
            } else {
                Polarity::Positive
            },
        })
        .collect();
    polymer
}

pub fn can_react(u1: &Unit, u2: &Unit) -> bool {
    match (
        u1.t.to_ascii_lowercase() == u2.t.to_ascii_lowercase(),
        u1.polarity,
        u2.polarity,
    ) {
        (true, Polarity::Positive, Polarity::Negative) => true,
        (true, Polarity::Negative, Polarity::Positive) => true,
        _ => false,
    }
}

pub fn reduce_polymer(polymer: &Polymer) -> Polymer {
    let mut polymer_cp = polymer.clone();
    let mut done = false;
    let mut last_index = 0;
    while !done {
//        println!("{}", show_polymer(&polymer_cp));
        done = true;
        for i in last_index..polymer_cp.len() - 1 {
            if can_react(&polymer_cp[i], &polymer_cp[i + 1]) {
//                println!("Removing {} {}", i, i + 1);
                polymer_cp.remove(i + 1);
                polymer_cp.remove(i);
                done = false;
                last_index = if i == 0 { 0 } else { i - 1 };
                break;
            }
        }
    }

    polymer_cp
}

pub fn part_one(polymer: &Polymer) -> usize {
    let reduced_polymer = reduce_polymer(polymer);
    //    println!("Reduced formula: {}", show_polymer(&reduced_polymer));
    reduced_polymer.len()
}

pub fn part_two(polymer: &Polymer) -> usize {
    let removed_polymers: Vec<Polymer> = "abcdefghijklmnopqrstuvwxyz"
        .chars()
        .map(|letter| {
            polymer
                .iter()
                .filter_map(|u| {
                    if !u.t.to_lowercase().eq(letter.to_lowercase()) {
                        Some(*u)
                    } else {
                        None
                    }
                })
                .collect()
        })
        .collect();

    let shortest_polymer_length = removed_polymers
        .iter()
        .map(|removed_polymer| reduce_polymer(removed_polymer).len())
        .min()
        .unwrap(); // Safe: there's at least one item

    shortest_polymer_length
}

fn main() -> Result<(), std::io::Error> {
    let data = read_data("input.txt")?;
    let polymer = parse_data(data);
    let p1 = part_one(&polymer);
    println!("Part one: {}", p1);
    let p2 = part_two(&polymer);
    println!("Part two: {}", p2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_parse_data() {
        let expected = "dabAcCaCBAcCcaDA";
        let actual = read_data("input_test.txt").unwrap();
        assert_eq!(expected, actual)
    }

}
