
mod evaluator;
mod symboltable;
mod structure;

use std::env;
use std::fs;
use std::io;
use std::io::Read;

use evaluator::EvaluationError;
use evaluator::Evaluator;

fn main() -> Result<(), EvaluationError> {
    let count = env::args().count();
    let evaluator = Evaluator::new();
    if count > 1 {
        // skip 1 because that's the executable name
        let results = env::args().skip(1).map(|argument| {
            let contents = fs::read_to_string(argument)
                .expect("Something went wrong reading the file");
            return evaluator.eval(contents);
        });
        for result in results {
            if !result.is_ok() {
                println!("{}", result.unwrap());
                return Ok(())
            }
        }
        return Ok(())
    }

    //if (isatty(fileno(stdin))) {
        //Repl r;
        //r.run();
        //return Ok(());
    //} else {

        // Read an expression from stdin
        let mut input = String::new();
        let result = io::stdin().read_to_string(&mut input);
        if !result.is_ok() {
            return Err(EvaluationError{});
        }
        let evaluation_result = evaluator.eval(input);
        if !evaluation_result.is_ok() {
            return Err(EvaluationError{});
        }
        println!("{}", evaluation_result.unwrap());
        return Ok(());

        
    //}

    // return Ok(());
}
