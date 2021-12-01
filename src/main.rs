use std::error;

#[derive(Debug)]
struct Acc {
    count: i64,
    prev_val: i64,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut input = include_str!("input.txt")
        .split('\n')
        .map(str::parse::<i64>)
        .map(Result::unwrap);

    let initial_val = input.next().unwrap();

    let initial_acc = Acc {
        count: 0,
        prev_val: initial_val,
    };

    let result = input.fold(initial_acc, |acc, val| {
        if val <= acc.prev_val {
            return Acc {
                count: acc.count,
                prev_val: val,
            };
        }

        Acc {
            count: acc.count + 1,
            prev_val: val,
        }
    });

    dbg!(result);

    Ok(())
}
