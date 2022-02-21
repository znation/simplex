mod astinput;
mod astnode;
mod errors;
mod evaluator;
mod parser;
mod stdlib;
mod structure;

use crate::structure::Empty;
use std::env;
use std::fs;
use std::io;
use std::io::Read;
use std::io::Write;
use std::io::stdin;
use std::io::stdout;

use errors::EvaluationError;
use evaluator::Evaluator;

use crate::structure::Backtrace;

fn main() -> Result<(), EvaluationError> {
    let count = env::args().count();
    let mut evaluator = Evaluator::new();
    if count > 1 {
        // skip 1 because that's the executable name
        let results = env::args().skip(1).map(|argument| {
            let contents =
                fs::read_to_string(argument).expect("Something went wrong reading the file");
            evaluator.eval(contents)
        });
        for result in results {
            if result.is_err() {
                println!("{}", result.unwrap());
                return Ok(());
            }
        }
        return Ok(());
    }

    if stdin_isatty() {
        // Run a REPL
        let mut eof = false;
        let mut evaluator = Evaluator::new();
        loop {
            print!("(simplex)> ");
            match stdout().flush() {
                Ok(_) => (),
                Err(e) => return Err(EvaluationError { message: format!("{}", e), backtrace: Backtrace::empty() })
            }
            let mut input = String::new();
            match stdin().read_line(&mut input) {
                Ok(count) => {
                    if count == 0 {
                        eof = true;
                    }
                },
                Err(e) => return Err(EvaluationError { message: format!("{}", e), backtrace: Backtrace::empty() })
            }
            let result = evaluator.eval(input);
            match result {
                Ok(value) => println!("{}", value),
                Err(e) => return Err(e)
            }
            if eof {
                break;
            }
        }
        Ok(())
    } else {
        // Read a single expression from stdin
        let mut input = String::new();
        let result = io::stdin().read_to_string(&mut input);
        if let Err(e) = result {
            return Err(EvaluationError {
                message: e.to_string(),
                backtrace: Backtrace::empty()
            });
        }
        let evaluation_result = evaluator.eval(input);
        match evaluation_result {
            Ok(value) => println!("{}", value),
            Err(e) => return Err(e),
        }
        Ok(())
    }
}

fn stdin_isatty() -> bool {
    unsafe { libc::isatty(libc::STDIN_FILENO) != 0 }
}