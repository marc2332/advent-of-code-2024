use std::fmt::{Debug, Write};

const INPUT: &str = include_str!("../input");

#[derive(Clone, Copy, PartialEq)]
enum Item {
    Num(usize),
    FreeSpace,
}

impl Debug for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Num(n) => f.write_str(&n.to_string()),
            Self::FreeSpace => f.write_char('.'),
        }
    }
}

fn main() {
    let data = INPUT
        .lines()
        .flat_map(|l| {
            l.chars()
                .map(|n| n.to_string().parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<_>>();

    let mut blocks = Vec::new();
    let mut blocks_len = 0;
    let mut last_was_zero = false;

    for num in data {
        if num == 0 {
            last_was_zero = true;
        }
        if blocks.is_empty()
            || blocks
                .last()
                .map(|block| matches!(block, Item::FreeSpace))
                .unwrap_or_default()
            || last_was_zero
        {
            if num != 0 {
                blocks.extend([Item::Num(blocks_len)].repeat(num as usize));
                blocks_len += 1;
                last_was_zero = false;
            }
        } else if !last_was_zero {
            blocks.extend([Item::FreeSpace].repeat(num as usize));
        }
    }

    p1(blocks.clone(), blocks_len);
    p2(blocks);
}

fn p2(mut blocks: Vec<Item>) {
    let mut block_stack = Vec::new();

    for (i, block) in blocks.clone().iter().rev().enumerate() {
        let swap = match block_stack.last() {
            None => false,
            Some((_, last_block)) if last_block == block => false,
            _ => true,
        };
        if swap {
            let block_stack = block_stack.drain(..).collect::<Vec<_>>();
            let mut space_stack = Vec::new();
            for (b, freespace) in blocks.clone()[0..blocks.len() - i].iter().enumerate() {
                if *freespace != Item::FreeSpace {
                    space_stack.clear();
                    continue;
                }
                space_stack.push(b);
                if space_stack.len() == block_stack.len() {
                    for (g, (n, _)) in block_stack.into_iter().enumerate() {
                        let j = space_stack.get(g).unwrap();
                        blocks.swap(n, *j);
                    }
                    break;
                }
            }
        }

        if *block != Item::FreeSpace {
            block_stack.push((blocks.len() - i - 1, *block));
        }
    }

    let total = blocks.into_iter().enumerate().fold(0, |acc, (i, e)| {
        if let Item::Num(n) = e {
            acc + n * i
        } else {
            acc
        }
    });

    println!("P2 {total:?}"); // 6413328569890
}

fn p1(mut blocks: Vec<Item>, blocks_len: usize) {
    for _ in 0..blocks.len() - blocks_len {
        let Some(block) = blocks.iter().rev().enumerate().find_map(|(i, b)| {
            if matches!(b, Item::Num(_)) {
                Some(blocks.len() - i - 1)
            } else {
                None
            }
        }) else {
            break;
        };

        let res = blocks.iter().enumerate().find_map(|(i, b)| {
            if matches!(b, Item::FreeSpace) {
                Some(i)
            } else {
                None
            }
        });
        let freespace = res.unwrap();

        blocks.swap(freespace, block);
    }

    let total = blocks.into_iter().enumerate().fold(0, |acc, (i, e)| {
        if let Item::Num(n) = e {
            acc + n * i
        } else {
            acc
        }
    });

    println!("P1 {total:?}"); // 6378826667552
}
