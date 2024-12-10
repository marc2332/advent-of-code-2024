use std::collections::HashSet;

const INPUT: &str = include_str!("../input");

fn main() {
    let data = INPUT
        .lines()
        .map(|l| {
            l.chars()
                .map(|n| n.to_string().parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<_>>();

    let mut total = 0;

    for (r, row) in data.iter().enumerate() {
        for (c, num) in row.iter().enumerate() {
            if *num == 0 {
                let mut solutions = HashSet::<(usize, usize)>::new();
                p1((r, c), &data, &mut solutions);
                let trails = solutions.len();
                total += trails;
            }
        }
    }

    println!("P2 {total}"); // 489

    let mut total = 0;

    for (r, row) in data.iter().enumerate() {
        for (c, num) in row.iter().enumerate() {
            if *num == 0 {
                let mut solutions = Vec::<(usize, usize)>::new();
                p2((r, c), &data, &mut solutions);
                let trails = solutions.len();
                total += trails;
            }
        }
    }

    println!("P2 {total}"); // 1086
}

fn p1((y, x): (usize, usize), data: &[Vec<u8>], solutions: &mut HashSet<(usize, usize)>) {
    let width = data[0].len() as i32;
    let height = data.len() as i32;
    let me = data[y][x];

    if me == 9 {
        solutions.insert((y, x));
        return;
    }

    let directions = [(-1i32, 0), (1, 0), (0, -1), (0, 1)];
    for (ver, hor) in directions {
        let new_y = y as i32 + ver;
        let new_x = x as i32 + hor;
        if new_x < 0 || new_y < 0 || new_x >= width || new_y >= height {
            continue;
        }
        let new_y = new_y as usize;
        let new_x = new_x as usize;
        let new_me = data[new_y][new_x];
        if new_me == me + 1 {
            p1((new_y, new_x), data, solutions);
        }
    }
}

fn p2((y, x): (usize, usize), data: &[Vec<u8>], solutions: &mut Vec<(usize, usize)>) {
    let width = data[0].len() as i32;
    let height = data.len() as i32;
    let me = data[y][x];

    if me == 9 {
        solutions.push((y, x));
        return;
    }

    let directions = [(-1i32, 0), (1, 0), (0, -1), (0, 1)];
    for (ver, hor) in directions {
        let new_y = y as i32 + ver;
        let new_x = x as i32 + hor;
        if new_x < 0 || new_y < 0 || new_x >= width || new_y >= height {
            continue;
        }
        let new_y = new_y as usize;
        let new_x = new_x as usize;
        let new_me = data[new_y][new_x];
        if new_me == me + 1 {
            p2((new_y, new_x), data, solutions);
        }
    }
}
