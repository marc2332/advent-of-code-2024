const INPUT: &str = include_str!("../input");

fn main() {
    let mut data = INPUT
        .lines()
        .flat_map(|l| {
            l.split_whitespace()
                .map(|n| n.to_string().parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<_>>();

    for blink in 0..25 {
        let mut offset = 0;
        for (i, stone) in data.clone().into_iter().enumerate() {
            let i = i + offset;
            let s = stone.to_string();
            match stone {
                0 => {
                    data[i] = 1;
                }
                _ if s.len() % 2 == 0 => {
                    let (first, second) = s.split_at(s.len() / 2);
                    data.insert(i, first.parse().unwrap());
                    *data.get_mut(i + 1).unwrap() = second.parse().unwrap();
                    offset += 1;
                }
                _ => {
                    data[i] = stone * 2024;
                }
            }
        }

        println!("{blink}: {}", data.len())
    }
}
