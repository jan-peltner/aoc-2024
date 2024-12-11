use std::fs::read_to_string;

fn parse_rules(rules: &str) -> Vec<(u32, u32)> {
    rules
        .split("\n")
        .map(|rule| {
            let nums: Vec<_> = rule
                .split("|")
                .map(|num| num.parse::<u32>().unwrap())
                .collect();
            (*nums.first().unwrap(), *nums.last().unwrap())
        })
        .collect()
}

fn parse_updates(updates: &str) -> Vec<Vec<u32>> {
    updates
        .split("\n")
        .map(|update| {
            update
                .split(",")
                .filter_map(|num| {
                    let parsed = num.parse::<u32>();
                    if parsed.is_ok() {
                        Some(parsed.unwrap())
                    } else {
                        None
                    }
                })
                .collect()
        })
        .collect()
}

fn get_left_pairs<'a>(needle: &'_ u32, rules: &'a Vec<(u32, u32)>) -> Vec<&'a (u32, u32)> {
    rules.iter().filter(|(lhs, _)| lhs == needle).collect()
}

fn get_right_pairs<'a>(needle: &'_ u32, rules: &'a Vec<(u32, u32)>) -> Vec<&'a (u32, u32)> {
    rules.iter().filter(|(_, rhs)| rhs == needle).collect()
}

fn is_valid_update(rules: &Vec<(u32, u32)>, update: &Vec<u32>) -> bool {
    for (i, num) in update.iter().enumerate() {
        let left_pairs = get_left_pairs(num, rules);
        let right_pairs = get_right_pairs(num, rules);
        for left in &update[..i] {
            if left_pairs.iter().any(|tup| &tup.1 == left) {
                return false;
            }
        }
        for right in &update[i..] {
            if right_pairs.iter().any(|tup| &tup.0 == right) {
                return false;
            }
        }
    }
    true
}

fn main() {
    let content = read_to_string("src/day05/input.txt").expect("Could not read input.txt");
    let content: Vec<&str> = content.split("\n\n").collect();
    let rules = content.first().unwrap();
    let updates = content.last().unwrap();

    let rules = parse_rules(rules);
    let mut updates = parse_updates(updates);
    updates.remove(updates.len() - 1);

    let successful_updates = updates
        .iter()
        .filter(|update| is_valid_update(&rules, update))
        .collect::<Vec<_>>();
    let sum: u32 = successful_updates
        .iter()
        .map(|update| update[update.len() / 2usize])
        .sum();
    dbg!(&sum);
}
