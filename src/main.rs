use std::error;

mod assignment_1;
mod assignment_2;
fn main() -> Result<(), Box<dyn error::Error>> {
    assignment_2::run().unwrap();

    Ok(())
}
