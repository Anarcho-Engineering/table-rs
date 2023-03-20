use std::fmt::Display;

pub struct Element {
    pub number: i32,
    pub name: String,
    pub symbol: String,
    pub mass: f32,
    pub electron_configuration: String,
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Number: {}\nName: {}\nSymbol: {}\nMass: {}\nElectron Configuration: {}",
            self.number, self.name, self.symbol, self.mass, self.electron_configuration
        )
    }
}
