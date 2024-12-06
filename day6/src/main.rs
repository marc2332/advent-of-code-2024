use std::collections::HashSet;

const INPUT: &str = include_str!("../input");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn next_dir(&mut self) {
        match self {
            Self::Up => *self = Self::Right,
            Self::Right => *self = Self::Down,
            Self::Down => *self = Self::Left,
            Self::Left => *self = Self::Up,
        }
    }
}

fn main() {
    let map = INPUT
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<char>>>();

    let initial_player = map
        .iter()
        .enumerate()
        .map(|(row, l)| {
            l.iter().enumerate().find_map(|(col, c)| {
                if *c == '^' {
                    Some((
                        row as i32,
                        col as i32,
                        match *c {
                            '^' => Direction::Up,
                            'v' => Direction::Down,
                            '<' => Direction::Left,
                            _ => Direction::Right,
                        },
                    ))
                } else {
                    None
                }
            })
        })
        .flatten()
        .collect::<Vec<(i32, i32, Direction)>>()[0];
    let mut player = initial_player.clone();
    let width_map = map.first().unwrap().len() as i32;
    let mut mapped = HashSet::<(i32, i32)>::new();

    loop {
        // Leave
        if (player.1 >= map.len() as i32)
            || (player.1 < 0)
            || (player.0 < 0)
            || (player.0 >= width_map)
        {
            break;
        }

        mapped.insert((player.0, player.1));

        // Next movement
        let next_pos = match player.2 {
            Direction::Up => (player.0 - 1, player.1),
            Direction::Down => (player.0 + 1, player.1),
            Direction::Left => (player.0, player.1 - 1),
            Direction::Right => (player.0, player.1 + 1),
        };

        if let Some(next_line) = map.get(next_pos.0 as usize) {
            if let Some(next_char) = next_line.get(next_pos.1 as usize) {
                if *next_char == '#' {
                    player.2.next_dir();
                    continue;
                }
            }
        }

        player.0 = next_pos.0;
        player.1 = next_pos.1;
    }

    println!("FIRST: {}", mapped.len()); // 5312

    let mut obstructions = 0;

    for pos in mapped {
        let mut mapped = HashSet::<(i32, i32, Direction)>::new();
        let mut player = initial_player.clone();
        let mut map = map.clone();
        map[pos.0 as usize][pos.1 as usize] = 'O';

        let stuck = loop {
            // Leave
            if (player.1 >= map.len() as i32)
                || (player.1 < 0)
                || (player.0 < 0)
                || (player.0 >= width_map)
            {
                break false;
            }

            // Next movement
            let next_pos = match player.2 {
                Direction::Up => (player.0 - 1, player.1),
                Direction::Down => (player.0 + 1, player.1),
                Direction::Left => (player.0, player.1 - 1),
                Direction::Right => (player.0, player.1 + 1),
            };

            if let Some(next_line) = map.get(next_pos.0 as usize) {
                if let Some(next_char) = next_line.get(next_pos.1 as usize) {
                    if *next_char == '#' || *next_char == 'O' {
                        if mapped.contains(&player) {
                            break true;
                        }

                        mapped.insert(player);
                        player.2.next_dir();
                        continue;
                    }
                }
            }

            mapped.insert(player);
            player.0 = next_pos.0;
            player.1 = next_pos.1;
        };

        if stuck {
            obstructions += 1;
        }
    }

    println!("SECOND: {obstructions}"); // 1748
}
