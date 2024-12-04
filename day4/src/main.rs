use std::{collections::BTreeSet, time::Instant};

const INPUT: &str = include_str!("../input");

fn main() {}

#[test]
fn second() {
    let lines = INPUT
        .lines()
        .into_iter()
        .map(|line| line.chars().filter(|c| *c != '\n').collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let instant = Instant::now();
    let mut combs = BTreeSet::new();

    for (row, line) in lines.iter().enumerate() {
        for (col, ch) in line.iter().enumerate() {
            if *ch == 'A' {
                let regions = [
                    [
                        Some(('M', -1, -1)),
                        None,
                        Some(('S', 1, -1)),
                        None,
                        Some(('A', 0, 0)),
                        None,
                        Some(('M', -1, 1)),
                        None,
                        Some(('S', 1, 1)),
                    ],
                    [
                        Some(('S', -1, -1)),
                        None,
                        Some(('S', 1, -1)),
                        None,
                        Some(('A', 0, 0)),
                        None,
                        Some(('M', -1, 1)),
                        None,
                        Some(('M', 1, 1)),
                    ],
                    [
                        Some(('M', -1, -1)),
                        None,
                        Some(('M', 1, -1)),
                        None,
                        Some(('A', 0, 0)),
                        None,
                        Some(('S', -1, 1)),
                        None,
                        Some(('S', 1, 1)),
                    ],
                    [
                        Some(('S', -1, -1)),
                        None,
                        Some(('M', 1, -1)),
                        None,
                        Some(('A', 0, 0)),
                        None,
                        Some(('S', -1, 1)),
                        None,
                        Some(('M', 1, 1)),
                    ],
                ];
                for region in regions {
                    let mut valid_positions = BTreeSet::new();
                    for position in region {
                        if let Some((expect, x, y)) = position {
                            if let Some((row, col)) =
                                row.checked_add_signed(y).zip(col.checked_add_signed(x))
                            {
                                if matches!(lines.get(row).map(|l| l.get(col)).flatten(), Some(ch) if *ch == expect)
                                {
                                    valid_positions.insert((row, col));
                                }
                            }
                        }
                    }
                    if valid_positions.len() == 5 {
                        combs.insert(valid_positions.into_iter().collect::<Vec<_>>());
                    }
                }
            }
        }
    }

    // 1978
    println!(
        "Found {:?} in {}ms",
        combs.len(),
        instant.elapsed().as_millis()
    );
}

#[test]
fn first() {
    let lines = INPUT
        .lines()
        .into_iter()
        .map(|line| line.chars().filter(|c| *c != '\n').collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let instant = Instant::now();
    let mut combs = BTreeSet::new();

    for (row, line) in lines.iter().enumerate() {
        for (col, ch) in line.iter().enumerate() {
            if *ch == 'X' {
                let regions = [
                    [[0, 0], [1, 0], [2, 0], [3, 0]],
                    [[0, 0], [-1, 0], [-2, 0], [-3, 0]],
                    [[0, 0], [0, -1], [0, -2], [0, -3]],
                    [[0, 0], [0, 1], [0, 2], [0, 3]],
                    [[0, 0], [1, 1], [2, 2], [3, 3]],
                    [[0, 0], [-1, 1], [-2, 2], [-3, 3]],
                    [[0, 0], [1, -1], [2, -2], [3, -3]],
                    [[0, 0], [-1, -1], [-2, -2], [-3, -3]],
                ];
                for region in regions {
                    let mut valid_positions = BTreeSet::new();
                    for (i, [x, y]) in region.into_iter().enumerate() {
                        let expect = ['X', 'M', 'A', 'S'][i];
                        if let Some((row, col)) =
                            row.checked_add_signed(y).zip(col.checked_add_signed(x))
                        {
                            if matches!(lines.get(row).map(|l| l.get(col)).flatten(), Some(ch) if *ch == expect)
                            {
                                valid_positions.insert((row, col));
                            }
                        }
                    }
                    if valid_positions.len() == 4 {
                        combs.insert(valid_positions.into_iter().collect::<Vec<_>>());
                    }
                }
            }
        }
    }

    // 2583
    println!(
        "Found {:?} in {}ms",
        combs.len(),
        instant.elapsed().as_millis()
    );
}
