#![no_main]
use libfuzzer_sys::fuzz_target;
use simplex;

// attempts to generate random strings,
// and pick out the ones that can parse correctly.

fuzz_target!(|data: &[u8]| {
    let s = match std::str::from_utf8(data) {
        Ok(s) => s,
        _ => return,
    };
    let _ = match simplex::parser::Parser::parse(s) {
        Ok(ast) => ast,
        _ => return,
    };
});
