use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../input");

fn main() {
    let data = INPUT
        .lines()
        .map(|l| {
            let chars = l.chars().collect::<Vec<char>>();
            chars
        })
        .collect::<Vec<_>>();
    let size = data.len() as i32;

    let mut map = HashMap::<(i32, i32), char>::new();

    for (li, line) in data.iter().enumerate() {
        for (ci, char) in line.iter().enumerate() {
            if *char != '.' {
                map.insert((li as i32, ci as i32), *char);
            }
        }
    }

    let mut antinodes_p1 = HashSet::<(i32, i32)>::new();
    let mut antinodes_p2 = HashSet::<(i32, i32)>::new();

    for ((li, ci), char) in map.iter() {
        let same_frequency = map.iter().filter_map(|((y, x), ch)| {
            if ch == char && y != li && x != ci {
                Some((y, x))
            } else {
                None
            }
        });

        for (y, x) in same_frequency {
            let diff_y = *li - y;
            let diff_x = *ci - x;
            for i in 0..size * size {
                antinodes_p2.insert((li + (diff_y * i as i32), ci + (diff_x * i as i32)));
            }
            antinodes_p1.insert((li + diff_y, ci + diff_x));
        }
    }

    let bound_filter =
        |a: &(i32, i32)| -> bool { a.1 >= 0 && a.1 < size && a.0 >= 0 && a.0 < size };

    let p1 = antinodes_p1.into_iter().filter(bound_filter);
    let p2 = antinodes_p2.into_iter().filter(bound_filter);

    println!("P1 {}", p1.count()); // 313
    println!("P2 {}", p2.count()); // 1064
}
