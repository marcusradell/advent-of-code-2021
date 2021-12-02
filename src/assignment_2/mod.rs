use std::error;

#[derive(Debug)]
struct Acc {
    count: i64,
    vals: Vec<i64>,
}

pub fn run() -> Result<(), Box<dyn error::Error>> {
    let input = include_str!("input.txt")
        .split('\n')
        .map(str::parse::<i64>)
        .map(Result::unwrap);

    let initial_acc = Acc {
        count: 0,
        vals: vec![],
    };

    let result = input.fold(initial_acc, |acc, val| {
        if acc.vals.len() < 3 {
            let mut vals = acc.vals;
            vals.insert(0, val);

            return Acc {
                count: acc.count,
                vals,
            };
        }

        let mut vals = acc.vals;

        if vals.len() > 3 {
            vals = vals[..3].into();
        }

        vals.insert(0, val);

        let prev_sum = vals[1] + vals[2] + vals[3];
        let next_sum = vals[0] + vals[1] + vals[2];

        let mut count = acc.count;

        dbg!(prev_sum);
        dbg!(next_sum);

        if prev_sum < next_sum {
            count += 1;
        }

        Acc { count, vals }
    });

    dbg!(result);

    Ok(())
}
