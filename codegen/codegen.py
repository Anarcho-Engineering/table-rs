import json

__author__ = """Pin Lee - (C) 2023"""

with open('./elements_by_number.json', 'r') as f:
    data = json.load(f)
    print(json.dumps(data, indent=4))

    with open('main.rs', 'w') as generated:

        generated.write("""#![allow(clippy::excessive_precision)]

use std::fmt::Display;

struct Element {
    number: i32,
    name: String,
    symbol: String,
    mass: f32,
    electron_configuration: String,
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Number: {}\\nName: {}\\nSymbol: {}\\nMass: {}\\nElectron Configuration: {}", 
            self.number, self.name, self.symbol, self.mass, self.electron_configuration)
    }
}

fn main() {
    let input: Vec<_> = std::env::args().collect();
    let arg = &input[1];
    let mut number_to_element = vec![];

    number_to_element.insert(0, Element { 
        number: 0, 
        name: "Default".to_owned(), 
        symbol:"ඞ".to_owned(),
        mass: 0_f32,
        electron_configuration: "".to_owned()
    });""")

        for key, value in data.items():
            # DO NOT TOUCH THE INDENTATION; YOU WILL BREAK EVERYTHING, YOU FOOL!!! 
            generated.write(f"""
        number_to_element.insert({value['atomic_number']}, Element {{ 
            number: {value['atomic_number']}, 
            name: "{value['element_name']}".to_owned(), 
            symbol:"{value['atomic_symbol']}".to_owned(),
            mass: {value['atomic_mass']}_f32,
            electron_configuration: "{value['electron_configuration']}".to_owned()
        }});
    """)

        generated.write("""
    // INPUT TO ELEMENT - match Identifier to Element (proc macro this)
    let output: &Element = match arg.as_str() {""")

        for key, value in data.items():
            generated.write(f""""{value['atomic_symbol']}" => &number_to_element[{value['atomic_number']}],
            "{value['element_name']}" => &number_to_element[{value['atomic_number']}],
            "{value['atomic_number']}" => &number_to_element[{value['atomic_number']}],
            """)  

        generated.write("""_ => panic!("Invalid element identifier."),
    };

    println!("{}", output);

}""")

        





