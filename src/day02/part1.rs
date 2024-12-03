use std::fs::read_to_string;

#[derive(Debug)]
enum Order {
    Ascending,
    Descending,
}

fn get_sort_direction(nums: &Vec<i32>) -> Option<Order> {
    let first = nums.first().expect("no number list");
    let last = nums.last().expect("no number list");
    if last > first {
        Some(Order::Ascending)
    } else if last < first {
        Some(Order::Descending)
    } else {
        None
    }
}

fn main() {
    let content = read_to_string("src/day02/input.txt").expect("Could not read input.txt");
    let numbers: Vec<Vec<i32>> = content
        .split("\n")
        .filter_map(|line| {
            let mapped = line
                .split_whitespace()
                .map(|x| x.parse::<i32>().expect("Could not convert string to int"))
                .collect::<Vec<i32>>();
            if mapped.len() > 0 {
                Some(mapped)
            } else {
                None
            }
        })
        .collect();

    let res: u32 = numbers
        .iter()
        .filter_map(|line| {
            if let Some(direction) = get_sort_direction(line) {
                let mut last: i32 = 0;
                for (idx, num) in line.iter().enumerate() {
                    if idx == 0 {
                        last = *num;
                        continue;
                    }
                    match direction {
                        Order::Descending => {
                            if *num < last && last.abs_diff(*num) < 4 {
                                last = *num;
                                continue;
                            } else {
                                return None;
                            }
                        }
                        Order::Ascending => {
                            if *num > last && last.abs_diff(*num) < 4 {
                                last = *num;
                                continue;
                            } else {
                                return None;
                            }
                        }
                    }
                }
                return Some(1);
            }
            return None;
        })
        .sum();
    println!("{res}");
}
