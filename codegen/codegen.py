import json

__author__ = """Pin Lee - (C) 2023"""

with open('./codegen/elements_by_number.json', 'r') as f:
    data = json.load(f)
    
    with open('src/elements.rs', 'w') as generated:
        generated.write("""#![allow(clippy::excessive_precision)]

use lazy_static::lazy_static;

pub mod element;
use crate::elements::element::Element;

lazy_static! {    
    static ref NUMBER_TO_ELEMENT: Vec<Element> = { 
        let mut tmp = vec![];

        tmp.insert(0, Element { 
            number: 0, 
            name: "Default".to_owned(), 
            symbol:"ඞ".to_owned(),
            mass: 0_f32,
            electron_configuration: "".to_owned()
        });
""")

        for key, value in data.items():
            # DO NOT TOUCH THE INDENTATION; YOU WILL BREAK EVERYTHING, YOU FOOL!!! 
            generated.write(f"""
        tmp.insert({value['atomic_number']}, Element {{ 
            number: {value['atomic_number']}, 
            name: "{value['element_name']}".to_owned(), 
            symbol: "{value['atomic_symbol']}".to_owned(),
            mass: {value['atomic_mass']}_f32,
            electron_configuration: "{value['electron_configuration']}".to_owned()
        }});
""")

        generated.write("""    tmp
    };
}

// INPUT TO ELEMENT - match Identifier to Element (proc macro this)
pub fn lookup(property: String) -> Result<&'static Element, &'static str> {
    match property.as_str() {
""")

        for key, value in data.items():
            generated.write(f"""        "{value['atomic_symbol']}" => Ok(&NUMBER_TO_ELEMENT[{value['atomic_number']}]),
        "{value['element_name']}" => Ok(&NUMBER_TO_ELEMENT[{value['atomic_number']}]),
        "{value['atomic_number']}" => Ok(&NUMBER_TO_ELEMENT[{value['atomic_number']}]),

""")  

        generated.write("""        _ => Err("Invalid element identifier."),
    }
}""")

        





