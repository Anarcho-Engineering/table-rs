#![allow(clippy::excessive_precision)]
use std::process::exit;

mod elements;
use crate::elements::lookup;

// https://stackoverflow.com/a/38406885
fn uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().chain(c).collect(),
    }
}

fn main() {
    let input: Vec<_> = std::env::args().collect();

    if input.len() < 2 {
        eprintln!("Please provide something to look up!");
        exit(1);
    }

    let arg = &input[1];
    match lookup(uppercase_first_letter(&arg.to_ascii_lowercase())) {
        Ok(description) => println!("{}", description),
        Err(error) => eprintln!("{}", error),
    }
}
