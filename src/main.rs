#![allow(clippy::excessive_precision)]
use std::process::exit;

mod elements;
use crate::elements::lookup;
use crate::elements::Element;

// https://stackoverflow.com/a/38406885
fn uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().chain(c).collect(),
    }
}

fn main() {
    let input: Vec<_> = std::env::args().collect::<Vec<_>>()[1..].to_vec();

    let mut results: Vec<&Element> = vec![];

    if input.is_empty() {
        eprintln!("Please provide something to look up!");
        exit(1);
    }

    for arg in input {
        match lookup(uppercase_first_letter(&arg.to_ascii_lowercase())) {
            Ok(element) => results.push(element),
            Err(error) => {
                eprintln!("{}", error);
                exit(1);
            }
        }
    }

    if results.len() == 1 {
        println!("{}", results[0]);
        return;
    }

    println!(
        "| {: <6} | {: <15} | {: <6} | {: <10} | {: <33} |\n+--------+-----------------+--------+------------+-----------------------------------+",
        "Number", "Name", "Symbol", "Mass", "Electron Configuration"
    );
    for result in results {
        println!(
            "| {: <6} | {: <15} | {: <6} | {: <10} | {: <33} |",
            result.number, result.name, result.symbol, result.mass, result.electron_configuration
        );
    }
}
