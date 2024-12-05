use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

const INPUT: &str = include_str!("../input");

fn main() {
    let content = INPUT.split("\n\n").collect::<Vec<&str>>();
    let rules_lines = content.first().unwrap();
    let updates_lines = content
        .get(1)
        .unwrap()
        .lines()
        .map(|l| {
            l.split(',')
                .map(|n| n.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<_>>();
    let mut rules = HashMap::<i32, Vec<i32>>::default();

    for rule in rules_lines.lines() {
        let rule = rule
            .split('|')
            .map(|n| n.parse().unwrap())
            .collect::<Vec<i32>>();
        let x = rule[0];
        let y = rule[1];
        rules.entry(x).or_default().push(y);
    }

    let mut middle_nums = Vec::<i32>::new();
    let mut unordered_updates = Vec::new();

    for nums in &updates_lines {
        let mut checked_nums = HashSet::new();
        let mut failed = false;
        for num in nums {
            if let Some(num_rules) = rules.get(&num) {
                if num_rules.iter().any(|r| checked_nums.contains(r)) {
                    failed = true;
                    break;
                }
            }
            checked_nums.insert(num);
        }

        if !failed {
            middle_nums.push(nums[nums.len() / 2]);
        } else {
            unordered_updates.push(nums.clone());
        }
    }

    let total = middle_nums.drain(..).sum::<i32>();

    println!("FIRST: {total:?}"); // 5948

    for mut nums in unordered_updates {
        nums.sort_by(|a, b| match rules.get(&a).cloned() {
            Some(rules) => {
                if rules.contains(&b) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }
            None => Ordering::Equal,
        });

        middle_nums.push(nums[nums.len() / 2]);
    }

    let total = middle_nums.into_iter().sum::<i32>();
    println!("SECOND: {total:?}"); // 3062
}
