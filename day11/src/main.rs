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
        for stone in data.drain(..).collect::<Vec<_>>().into_iter() {
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

        println!("{blink}: {:?}", data.len())
    }
}
