use std::fs::read_to_string;
fn main() {
    let content = read_to_string("src/day01/input.txt").expect("Could not read input.txt");
    let numbers: Vec<Vec<i32>> = content
        .split("\n")
        .filter_map(|line| {
            let mapped = line
                .split_whitespace()
                .take(2)
                .map(|x| x.parse::<i32>().expect("Could not convert string to int"))
                .collect::<Vec<i32>>();
            if mapped.len() == 2 {
                Some(mapped)
            } else {
                None
            }
        })
        .collect();

    let mut left = numbers
        .iter()
        .map(|nums| nums[0].clone())
        .collect::<Vec<i32>>();

    let mut right = numbers
        .iter()
        .map(|nums| nums[1].clone())
        .collect::<Vec<i32>>();

    left.sort();
    right.sort();

    let res = left
        .iter()
        .zip(right.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum::<u32>();

    println!("{res}");
}
