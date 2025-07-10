use std::error::Error;

use crate::parsing::parse;

pub mod evaluation;
pub mod parsing;
pub mod types;
pub mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    let input: &str = "(10*11)/2";

    println!("< {}", input);

    let (_, e) = parse(input)?;

    print!("> ");
    utils::print(&e);
    println!(" = {}", evaluation::eval(&e));

    Ok(())
}
