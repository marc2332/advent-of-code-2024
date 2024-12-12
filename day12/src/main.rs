use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../input");

fn main() {
    let data = INPUT
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    p1(data.clone());
    p2(data);
}

fn p1(data: Vec<Vec<char>>) {
    let mut zones = HashMap::<char, Vec<HashSet<(i32, i32)>>>::new();
    let width = data[0].len();
    let height = data.len();

    for (row, line) in data.iter().enumerate() {
        for (col, c) in line.iter().enumerate() {
            let c_zone = zones.entry(*c).or_default();
            if c_zone
                .iter()
                .any(|reg| reg.contains(&(row as i32, col as i32)))
            {
                continue;
            }
            let mut zone = HashSet::from([(row as i32, col as i32)]);
            map_zone(row, col, &data, width, height, &mut zone);
            c_zone.push(zone);
        }
    }

    let mut total_price = 0;
    for (_c, zone) in zones {
        for region in zone.iter() {
            let area = region;
            let perimeter = perimeter(region).len();
            let price = area.len() * perimeter;
            total_price += price;
        }
    }

    println!("p1: {total_price}"); // 1363682
}

fn map_zone(
    row: usize,
    col: usize,
    data: &Vec<Vec<char>>,
    width: usize,
    height: usize,
    zone: &mut HashSet<(i32, i32)>,
) {
    let c = data[row][col];
    let directions = [(-1i32, 0), (1, 0), (0, -1), (0, 1)];
    for (hor, ver) in directions {
        let x = col as i32 + hor;
        let y = row as i32 + ver;
        if x < 0 || x >= width as i32 || y < 0 || y >= height as i32 {
            continue;
        }

        let side_c = data[y as usize][x as usize];

        if side_c == c && !zone.contains(&(y as i32, x as i32)) {
            zone.insert((y as i32, x as i32));
            map_zone(y as usize, x as usize, data, width, height, zone);
        }
    }
}

fn perimeter(data: &HashSet<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut empty = Vec::<(i32, i32)>::new();
    for (y, x) in data.iter() {
        let x = *x as i32;
        let y = *y as i32;
        let sides = [(-1i32, 0), (1, 0), (0, -1), (0, 1)];
        for (v, h) in sides {
            let x2 = x + h as i32;
            let y2 = y + v as i32;
            if !data.contains(&(y2, x2)) {
                empty.push((y2, x2));
            }
        }

        empty.retain(|a| *a != (y, x));
    }
    empty
}

fn num_sides(_data: &HashSet<(i32, i32)>) -> usize {
    0
}

fn p2(data: Vec<Vec<char>>) {
    let mut zones = HashMap::<char, Vec<HashSet<(i32, i32)>>>::new();
    let width = data[0].len();
    let height = data.len();

    for (row, line) in data.iter().enumerate() {
        for (col, c) in line.iter().enumerate() {
            let c_zone = zones.entry(*c).or_default();
            if c_zone
                .iter()
                .any(|reg| reg.contains(&(row as i32, col as i32)))
            {
                continue;
            }
            let mut zone = HashSet::from([(row as i32, col as i32)]);
            map_zone(row, col, &data, width, height, &mut zone);
            c_zone.push(zone);
        }
    }

    let mut total_price = 0;
    for (_c, zone) in zones {
        for region in zone.iter() {
            let area = region;
            let sides = num_sides(region);
            let price = area.len() * sides;
            total_price += price;
        }
    }

    println!("p2: {total_price}");
}
