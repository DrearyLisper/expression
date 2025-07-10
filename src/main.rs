use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;

pub mod evaluation;
pub mod parsing;
pub mod types;
pub mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rl = DefaultEditor::new()?;

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => match parsing::parse(&line) {
                Ok((_, expr)) => {
                    print!("> ");
                    utils::print(&expr);
                    println!(" = {}", evaluation::eval(&expr));
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}
