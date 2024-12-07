const INPUT: &str = include_str!("../input");

fn main() {
    // let start = std::time::Instant::now();
    let equations = INPUT
        .lines()
        .map(|l| {
            let mut split = l.split(':').into_iter();
            let test_value = split.nth(0).unwrap();
            let numbers = split.nth(0).unwrap().trim();
            (
                test_value.parse::<i128>().unwrap(),
                numbers
                    .split(' ')
                    .map(|n| n.parse::<i128>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    let mut p1 = 0;
    'equation: for (test_value, numbers) in equations.clone() {
        let mut values = vec![numbers[0]];
        for (i, num) in numbers[1..].into_iter().enumerate() {
            for val in values.drain(..).collect::<Vec<_>>() {
                let by_sum = val + num;
                let by_mul = val * num;
                let is_last = i == numbers.len() - 2;

                if is_last && (by_sum == test_value || by_mul == test_value) {
                    p1 += test_value;
                    continue 'equation;
                } else {
                    values.extend([by_sum, by_mul]);
                }
            }
        }
    }

    // println!("p1 {}ms", start.elapsed().as_millis());

    let mut p2 = 0;
    'equation: for (test_value, numbers) in equations {
        let mut values = vec![numbers[0]];
        for (i, num) in numbers[1..].into_iter().enumerate() {
            for val in values.drain(..).collect::<Vec<_>>() {
                let by_sum = val + num;
                let by_mul = val * num;
                let by_conc = format!("{val}{num}").parse::<i128>().unwrap();
                let is_last = i == numbers.len() - 2;

                if is_last
                    && (by_sum == test_value || by_mul == test_value || by_conc == test_value)
                {
                    p2 += test_value;
                    continue 'equation;
                } else {
                    values.extend([by_sum, by_mul, by_conc]);
                }
            }
        }
    }

    // println!("p2 {}ms", start.elapsed().as_millis());

    println!("FIRST: {p1}"); // 945512582195
    println!("SECOND: {p2}"); // 271691107779347
}
