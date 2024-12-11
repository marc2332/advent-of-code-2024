use std::collections::HashMap;

const INPUT: &str = include_str!("../input");

fn main() {
    let data = INPUT
        .lines()
        .flat_map(|l| {
            l.split_whitespace()
                .map(|n| n.to_string().parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<_>>();

    p1(data.clone());
    p2(data);
}

fn p1(mut data: Vec<i64>) {
    for _ in 0..25 {
        for stone in std::mem::take(&mut data) {
            let s = stone.to_string();
            match stone {
                0 => {
                    data.push(1);
                }
                _ if s.len() % 2 == 0 => {
                    let (first, second) = s.split_at(s.len() / 2);
                    data.push(first.parse().unwrap());
                    data.push(second.parse().unwrap());
                }
                _ => {
                    data.push(stone * 2024);
                }
            }
        }
    }
    println!("p1: {:?}", data.len()) // 203609
}

fn p2(data: Vec<i64>) {
    let mut nums = HashMap::<i64, usize>::new();

    for num in data {
        *nums.entry(num).or_default() += 1;
    }

    for _ in 0..75 {
        for (stone, amm) in std::mem::take(&mut nums) {
            let s = stone.to_string();
            match stone {
                0 => {
                    *nums.entry(1).or_default() += amm;
                }
                _ if s.len() % 2 == 0 => {
                    let (first, second) = s.split_at(s.len() / 2);
                    *nums.entry(first.parse().unwrap()).or_default() += amm;
                    *nums.entry(second.parse().unwrap()).or_default() += amm;
                }
                _ => {
                    *nums.entry(stone * 2024).or_default() += amm;
                }
            }
        }
    }

    let total = nums.iter().fold(0, |acc, (_, v)| acc + *v);
    println!("p2: {:?}", total); // 240954878211138
}
