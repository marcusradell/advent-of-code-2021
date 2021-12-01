use std::error;

mod assignment_1;

fn main() -> Result<(), Box<dyn error::Error>> {
    assignment_1::run().unwrap();

    Ok(())
}
