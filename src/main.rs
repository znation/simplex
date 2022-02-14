mod astinput;
mod astnode;
mod errors;
mod evaluator;
mod parser;
mod stdlib;
mod structure;

use std::env;
use std::fs;
use std::io;
use std::io::Read;

use errors::EvaluationError;
use evaluator::Evaluator;

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

    //if (isatty(fileno(stdin))) {
    //Repl r;
    //r.run();
    //return Ok(());
    //} else {

    // Read an expression from stdin
    let mut input = String::new();
    let result = io::stdin().read_to_string(&mut input);
    if let Err(e) = result {
        return Err(EvaluationError {
            message: e.to_string(),
        });
    }
    let evaluation_result = evaluator.eval(input);
    match evaluation_result {
        Ok(value) => println!("{}", value),
        Err(e) => return Err(e),
    }
    Ok(())

    //}

    // return Ok(());
}
