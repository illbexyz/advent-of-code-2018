extern crate itertools;
extern crate regex;

use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn parse_data(data: &String) -> Vec<Point> {
    let re = Regex::new(r"(?m)^(\d+),\s(\d+)").unwrap();
    re.captures_iter(data)
        .map(|cap| Point {
            x: cap[2].parse::<i32>().unwrap(),
            y: cap[1].parse::<i32>().unwrap(),
        })
        .collect()
}

fn bottom_right(points: &Vec<Point>) -> Point {
    let mut max_x = 0;
    let mut max_y = 0;
    for point in points.iter() {
        if point.x > max_x {
            max_x = point.x
        }
        if point.y > max_y {
            max_y = point.y
        }
    }
    Point {
        x: max_y + 2,
        y: max_x + 1,
    }
}

fn distance(p1: &Point, p2: &Point) -> i32 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

fn part_one(points: &Vec<Point>) -> i32 {
    let bounding_box = bottom_right(points);
    let mut grid = vec![vec![0; bounding_box.x as usize]; bounding_box.y as usize];

    for (i, p) in points.iter().enumerate() {
        grid[p.x as usize][p.y as usize] = i + 1;
    }

    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            let distance_from_points: Vec<(i32, usize)> = points
                .iter()
                .enumerate()
                .map(|(idx, point)| {
                    (
                        distance(
                            &Point {
                                x: x as i32,
                                y: y as i32,
                            },
                            &point,
                        ),
                        idx,
                    )
                })
                .sorted();

            let nearest_point_idx = match (distance_from_points[0], distance_from_points[1]) {
                ((dist1, idx1), (dist2, _idx2)) if dist1 < dist2 => idx1 + 1,
                ((dist1, _idx1), (dist2, _idx2)) if dist1 == dist2 => 0,
                _ => panic!("dist2 can't be greater than dist1"),
            };
            grid[x][y] = nearest_point_idx;
        }
    }

    // Check borders for infinite areas
    println!("{:?}", bounding_box);
    let not_infinite_areas_idxs: Vec<usize> = points
        .iter()
        .enumerate()
        .filter_map(|(idx, _point)| {
            let grid_idx = idx + 1;

            let mut keep = true;
            for x in 0..grid.len() {
                if grid[x][0] == grid_idx || grid[x][grid[0].len() - 1] == grid_idx {
                    keep = false
                }
            }

            for y in 0..grid[0].len() {
                if grid[0][y] == grid_idx || grid[grid.len() - 1][y] == grid_idx {
                    keep = false
                }
            }

            match keep {
                true => Some(idx + 1),
                false => None,
            }
        })
        .collect();

    let mut count_occurrences = HashMap::new();

    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            let key = grid[x][y];
            let prev_val = match count_occurrences.get(&key) {
                Some(val) => *val,
                None => 0,
            };
            count_occurrences.insert(key, prev_val + 1);
        }
    }

    let max_areas = count_occurrences
        .iter()
        .filter(|(idx, _val)| not_infinite_areas_idxs.contains(idx))
        .max_by_key(|(_idx, val)| *val);

    match max_areas {
        Some((_, max_area)) => *max_area,
        None => panic!("Not max area found"),
    }
}

fn part_two(points: &Vec<Point>, limit: i32) -> i32 {
    let btm_rght = bottom_right(points);
    let mut grid = vec![vec![0; btm_rght.x as usize]; btm_rght.y as usize];

    for (i, p) in points.iter().enumerate() {
        grid[p.x as usize][p.y as usize] = i + 1;
    }

    let mut cnt = 0;

    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            let distance_from_points: Vec<(i32, usize)> = points
                .iter()
                .enumerate()
                .map(|(idx, point)| {
                    (
                        distance(
                            &Point {
                                x: x as i32,
                                y: y as i32,
                            },
                            &point,
                        ),
                        idx,
                    )
                })
                .collect();

            let sum_of_distances = distance_from_points
                .iter()
                .fold(0, |acc, (dist, _)| acc + dist);
            if sum_of_distances < limit {
                cnt += 1;
            }
        }
    }

    cnt
}

fn main() -> Result<(), std::io::Error> {
    let data = std::fs::read_to_string("input_test.txt")?;
    let points = parse_data(&data);
    let p1 = part_one(&points);
    println!("Part one: {}", p1);
    let p2 = part_two(&points, 10000);
    println!("Part two: {}", p2);
    Ok(())
}
