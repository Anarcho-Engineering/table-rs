#![allow(clippy::excessive_precision)]

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
        write!(f, "Number: {}\nName: {}\nSymbol: {}\nMass: {}\nElectron Configuration: {}", 
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
    });
        number_to_element.insert(1, Element { 
            number: 1, 
            name: "Hydrogen".to_owned(), 
            symbol:"H".to_owned(),
            mass: 1.0080_f32,
            electron_configuration: "1s1".to_owned()
        });
    
        number_to_element.insert(2, Element { 
            number: 2, 
            name: "Helium".to_owned(), 
            symbol:"He".to_owned(),
            mass: 4.00260_f32,
            electron_configuration: "1s2".to_owned()
        });
    
        number_to_element.insert(3, Element { 
            number: 3, 
            name: "Lithium".to_owned(), 
            symbol:"Li".to_owned(),
            mass: 7.0_f32,
            electron_configuration: "[He]2s1".to_owned()
        });
    
        number_to_element.insert(4, Element { 
            number: 4, 
            name: "Beryllium".to_owned(), 
            symbol:"Be".to_owned(),
            mass: 9.012183_f32,
            electron_configuration: "[He]2s2".to_owned()
        });
    
        number_to_element.insert(5, Element { 
            number: 5, 
            name: "Boron".to_owned(), 
            symbol:"B".to_owned(),
            mass: 10.81_f32,
            electron_configuration: "[He]2s2 2p1".to_owned()
        });
    
        number_to_element.insert(6, Element { 
            number: 6, 
            name: "Carbon".to_owned(), 
            symbol:"C".to_owned(),
            mass: 12.011_f32,
            electron_configuration: "[He]2s2 2p2".to_owned()
        });
    
        number_to_element.insert(7, Element { 
            number: 7, 
            name: "Nitrogen".to_owned(), 
            symbol:"N".to_owned(),
            mass: 14.007_f32,
            electron_configuration: "[He] 2s2 2p3".to_owned()
        });
    
        number_to_element.insert(8, Element { 
            number: 8, 
            name: "Oxygen".to_owned(), 
            symbol:"O".to_owned(),
            mass: 15.999_f32,
            electron_configuration: "[He]2s2 2p4".to_owned()
        });
    
        number_to_element.insert(9, Element { 
            number: 9, 
            name: "Fluorine".to_owned(), 
            symbol:"F".to_owned(),
            mass: 18.99840316_f32,
            electron_configuration: "[He]2s2 2p5".to_owned()
        });
    
        number_to_element.insert(10, Element { 
            number: 10, 
            name: "Neon".to_owned(), 
            symbol:"Ne".to_owned(),
            mass: 20.180_f32,
            electron_configuration: "[He]2s2 2p6".to_owned()
        });
    
        number_to_element.insert(11, Element { 
            number: 11, 
            name: "Sodium".to_owned(), 
            symbol:"Na".to_owned(),
            mass: 22.9897693_f32,
            electron_configuration: "[Ne]3s1".to_owned()
        });
    
        number_to_element.insert(12, Element { 
            number: 12, 
            name: "Magnesium".to_owned(), 
            symbol:"Mg".to_owned(),
            mass: 24.305_f32,
            electron_configuration: "[Ne]3s2".to_owned()
        });
    
        number_to_element.insert(13, Element { 
            number: 13, 
            name: "Aluminum".to_owned(), 
            symbol:"Al".to_owned(),
            mass: 26.981538_f32,
            electron_configuration: "[Ne]3s2 3p1".to_owned()
        });
    
        number_to_element.insert(14, Element { 
            number: 14, 
            name: "Silicon".to_owned(), 
            symbol:"Si".to_owned(),
            mass: 28.085_f32,
            electron_configuration: "[Ne]3s2 3p2".to_owned()
        });
    
        number_to_element.insert(15, Element { 
            number: 15, 
            name: "Phosphorus".to_owned(), 
            symbol:"P".to_owned(),
            mass: 30.97376200_f32,
            electron_configuration: "[Ne]3s2 3p3".to_owned()
        });
    
        number_to_element.insert(16, Element { 
            number: 16, 
            name: "Sulfur".to_owned(), 
            symbol:"S".to_owned(),
            mass: 32.07_f32,
            electron_configuration: "[Ne]3s2 3p4".to_owned()
        });
    
        number_to_element.insert(17, Element { 
            number: 17, 
            name: "Chlorine".to_owned(), 
            symbol:"Cl".to_owned(),
            mass: 35.45_f32,
            electron_configuration: "[Ne]3s2 3p5".to_owned()
        });
    
        number_to_element.insert(18, Element { 
            number: 18, 
            name: "Argon".to_owned(), 
            symbol:"Ar".to_owned(),
            mass: 39.9_f32,
            electron_configuration: "[Ne]3s2 3p6".to_owned()
        });
    
        number_to_element.insert(19, Element { 
            number: 19, 
            name: "Potassium".to_owned(), 
            symbol:"K".to_owned(),
            mass: 39.0983_f32,
            electron_configuration: "[Ar]4s1".to_owned()
        });
    
        number_to_element.insert(20, Element { 
            number: 20, 
            name: "Calcium".to_owned(), 
            symbol:"Ca".to_owned(),
            mass: 40.08_f32,
            electron_configuration: "[Ar]4s2".to_owned()
        });
    
        number_to_element.insert(21, Element { 
            number: 21, 
            name: "Scandium".to_owned(), 
            symbol:"Sc".to_owned(),
            mass: 44.95591_f32,
            electron_configuration: "[Ar]4s2 3d1".to_owned()
        });
    
        number_to_element.insert(22, Element { 
            number: 22, 
            name: "Titanium".to_owned(), 
            symbol:"Ti".to_owned(),
            mass: 47.867_f32,
            electron_configuration: "[Ar]4s2 3d2".to_owned()
        });
    
        number_to_element.insert(23, Element { 
            number: 23, 
            name: "Vanadium".to_owned(), 
            symbol:"V".to_owned(),
            mass: 50.9415_f32,
            electron_configuration: "[Ar]4s2 3d3".to_owned()
        });
    
        number_to_element.insert(24, Element { 
            number: 24, 
            name: "Chromium".to_owned(), 
            symbol:"Cr".to_owned(),
            mass: 51.996_f32,
            electron_configuration: "[Ar]3d5 4s1".to_owned()
        });
    
        number_to_element.insert(25, Element { 
            number: 25, 
            name: "Manganese".to_owned(), 
            symbol:"Mn".to_owned(),
            mass: 54.93804_f32,
            electron_configuration: "[Ar]4s2 3d5".to_owned()
        });
    
        number_to_element.insert(26, Element { 
            number: 26, 
            name: "Iron".to_owned(), 
            symbol:"Fe".to_owned(),
            mass: 55.84_f32,
            electron_configuration: "[Ar]4s2 3d6".to_owned()
        });
    
        number_to_element.insert(27, Element { 
            number: 27, 
            name: "Cobalt".to_owned(), 
            symbol:"Co".to_owned(),
            mass: 58.93319_f32,
            electron_configuration: "[Ar]4s2 3d7".to_owned()
        });
    
        number_to_element.insert(28, Element { 
            number: 28, 
            name: "Nickel".to_owned(), 
            symbol:"Ni".to_owned(),
            mass: 58.693_f32,
            electron_configuration: "[Ar]4s2 3d8".to_owned()
        });
    
        number_to_element.insert(29, Element { 
            number: 29, 
            name: "Copper".to_owned(), 
            symbol:"Cu".to_owned(),
            mass: 63.55_f32,
            electron_configuration: "[Ar]4s1 3d10".to_owned()
        });
    
        number_to_element.insert(30, Element { 
            number: 30, 
            name: "Zinc".to_owned(), 
            symbol:"Zn".to_owned(),
            mass: 65.4_f32,
            electron_configuration: "[Ar]4s2 3d10".to_owned()
        });
    
        number_to_element.insert(31, Element { 
            number: 31, 
            name: "Gallium".to_owned(), 
            symbol:"Ga".to_owned(),
            mass: 69.723_f32,
            electron_configuration: "[Ar]4s2 3d10 4p1".to_owned()
        });
    
        number_to_element.insert(32, Element { 
            number: 32, 
            name: "Germanium".to_owned(), 
            symbol:"Ge".to_owned(),
            mass: 72.63_f32,
            electron_configuration: "[Ar]4s2 3d10 4p2".to_owned()
        });
    
        number_to_element.insert(33, Element { 
            number: 33, 
            name: "Arsenic".to_owned(), 
            symbol:"As".to_owned(),
            mass: 74.92159_f32,
            electron_configuration: "[Ar]4s2 3d10 4p3".to_owned()
        });
    
        number_to_element.insert(34, Element { 
            number: 34, 
            name: "Selenium".to_owned(), 
            symbol:"Se".to_owned(),
            mass: 78.97_f32,
            electron_configuration: "[Ar]4s2 3d10 4p4".to_owned()
        });
    
        number_to_element.insert(35, Element { 
            number: 35, 
            name: "Bromine".to_owned(), 
            symbol:"Br".to_owned(),
            mass: 79.90_f32,
            electron_configuration: "[Ar]4s2 3d10 4p5".to_owned()
        });
    
        number_to_element.insert(36, Element { 
            number: 36, 
            name: "Krypton".to_owned(), 
            symbol:"Kr".to_owned(),
            mass: 83.80_f32,
            electron_configuration: "[Ar]4s2 3d10 4p6".to_owned()
        });
    
        number_to_element.insert(37, Element { 
            number: 37, 
            name: "Rubidium".to_owned(), 
            symbol:"Rb".to_owned(),
            mass: 85.468_f32,
            electron_configuration: "[Kr]5s1".to_owned()
        });
    
        number_to_element.insert(38, Element { 
            number: 38, 
            name: "Strontium".to_owned(), 
            symbol:"Sr".to_owned(),
            mass: 87.62_f32,
            electron_configuration: "[Kr]5s2".to_owned()
        });
    
        number_to_element.insert(39, Element { 
            number: 39, 
            name: "Yttrium".to_owned(), 
            symbol:"Y".to_owned(),
            mass: 88.90584_f32,
            electron_configuration: "[Kr]5s2 4d1".to_owned()
        });
    
        number_to_element.insert(40, Element { 
            number: 40, 
            name: "Zirconium".to_owned(), 
            symbol:"Zr".to_owned(),
            mass: 91.22_f32,
            electron_configuration: "[Kr]5s2 4d2".to_owned()
        });
    
        number_to_element.insert(41, Element { 
            number: 41, 
            name: "Niobium".to_owned(), 
            symbol:"Nb".to_owned(),
            mass: 92.90637_f32,
            electron_configuration: "[Kr]5s1 4d4".to_owned()
        });
    
        number_to_element.insert(42, Element { 
            number: 42, 
            name: "Molybdenum".to_owned(), 
            symbol:"Mo".to_owned(),
            mass: 95.95_f32,
            electron_configuration: "[Kr]5s1 4d5".to_owned()
        });
    
        number_to_element.insert(43, Element { 
            number: 43, 
            name: "Technetium".to_owned(), 
            symbol:"Tc".to_owned(),
            mass: 96.90636_f32,
            electron_configuration: "[Kr]5s2 4d5".to_owned()
        });
    
        number_to_element.insert(44, Element { 
            number: 44, 
            name: "Ruthenium".to_owned(), 
            symbol:"Ru".to_owned(),
            mass: 101.1_f32,
            electron_configuration: "[Kr]5s1 4d7".to_owned()
        });
    
        number_to_element.insert(45, Element { 
            number: 45, 
            name: "Rhodium".to_owned(), 
            symbol:"Rh".to_owned(),
            mass: 102.9055_f32,
            electron_configuration: "[Kr]5s1 4d8".to_owned()
        });
    
        number_to_element.insert(46, Element { 
            number: 46, 
            name: "Palladium".to_owned(), 
            symbol:"Pd".to_owned(),
            mass: 106.42_f32,
            electron_configuration: "[Kr]4d10".to_owned()
        });
    
        number_to_element.insert(47, Element { 
            number: 47, 
            name: "Silver".to_owned(), 
            symbol:"Ag".to_owned(),
            mass: 107.868_f32,
            electron_configuration: "[Kr]5s1 4d10".to_owned()
        });
    
        number_to_element.insert(48, Element { 
            number: 48, 
            name: "Cadmium".to_owned(), 
            symbol:"Cd".to_owned(),
            mass: 112.41_f32,
            electron_configuration: "[Kr]5s2 4d10".to_owned()
        });
    
        number_to_element.insert(49, Element { 
            number: 49, 
            name: "Indium".to_owned(), 
            symbol:"In".to_owned(),
            mass: 114.818_f32,
            electron_configuration: "[Kr]5s2 4d10 5p1".to_owned()
        });
    
        number_to_element.insert(50, Element { 
            number: 50, 
            name: "Tin".to_owned(), 
            symbol:"Sn".to_owned(),
            mass: 118.71_f32,
            electron_configuration: "[Kr]5s2 4d10 5p2".to_owned()
        });
    
        number_to_element.insert(51, Element { 
            number: 51, 
            name: "Antimony".to_owned(), 
            symbol:"Sb".to_owned(),
            mass: 121.760_f32,
            electron_configuration: "[Kr]5s2 4d10 5p3".to_owned()
        });
    
        number_to_element.insert(52, Element { 
            number: 52, 
            name: "Tellurium".to_owned(), 
            symbol:"Te".to_owned(),
            mass: 127.6_f32,
            electron_configuration: "[Kr]5s2 4d10 5p4".to_owned()
        });
    
        number_to_element.insert(53, Element { 
            number: 53, 
            name: "Iodine".to_owned(), 
            symbol:"I".to_owned(),
            mass: 126.9045_f32,
            electron_configuration: "[Kr]5s2 4d10 5p5".to_owned()
        });
    
        number_to_element.insert(54, Element { 
            number: 54, 
            name: "Xenon".to_owned(), 
            symbol:"Xe".to_owned(),
            mass: 131.29_f32,
            electron_configuration: "[Kr]5s2 4d10 5p6".to_owned()
        });
    
        number_to_element.insert(55, Element { 
            number: 55, 
            name: "Cesium".to_owned(), 
            symbol:"Cs".to_owned(),
            mass: 132.9054520_f32,
            electron_configuration: "[Xe]6s1".to_owned()
        });
    
        number_to_element.insert(56, Element { 
            number: 56, 
            name: "Barium".to_owned(), 
            symbol:"Ba".to_owned(),
            mass: 137.33_f32,
            electron_configuration: "[Xe]6s2".to_owned()
        });
    
        number_to_element.insert(57, Element { 
            number: 57, 
            name: "Lanthanum".to_owned(), 
            symbol:"La".to_owned(),
            mass: 138.9055_f32,
            electron_configuration: "[Xe]6s2 5d1".to_owned()
        });
    
        number_to_element.insert(58, Element { 
            number: 58, 
            name: "Cerium".to_owned(), 
            symbol:"Ce".to_owned(),
            mass: 140.116_f32,
            electron_configuration: "[Xe]6s2 4f1 5d1".to_owned()
        });
    
        number_to_element.insert(59, Element { 
            number: 59, 
            name: "Praseodymium".to_owned(), 
            symbol:"Pr".to_owned(),
            mass: 140.90766_f32,
            electron_configuration: "[Xe]6s2 4f3".to_owned()
        });
    
        number_to_element.insert(60, Element { 
            number: 60, 
            name: "Neodymium".to_owned(), 
            symbol:"Nd".to_owned(),
            mass: 144.24_f32,
            electron_configuration: "[Xe]6s2 4f4".to_owned()
        });
    
        number_to_element.insert(61, Element { 
            number: 61, 
            name: "Promethium".to_owned(), 
            symbol:"Pm".to_owned(),
            mass: 144.91276_f32,
            electron_configuration: "[Xe]6s2 4f5".to_owned()
        });
    
        number_to_element.insert(62, Element { 
            number: 62, 
            name: "Samarium".to_owned(), 
            symbol:"Sm".to_owned(),
            mass: 150.4_f32,
            electron_configuration: "[Xe]6s2 4f6".to_owned()
        });
    
        number_to_element.insert(63, Element { 
            number: 63, 
            name: "Europium".to_owned(), 
            symbol:"Eu".to_owned(),
            mass: 151.964_f32,
            electron_configuration: "[Xe]6s2 4f7".to_owned()
        });
    
        number_to_element.insert(64, Element { 
            number: 64, 
            name: "Gadolinium".to_owned(), 
            symbol:"Gd".to_owned(),
            mass: 157.2_f32,
            electron_configuration: "[Xe]6s2 4f7 5d1".to_owned()
        });
    
        number_to_element.insert(65, Element { 
            number: 65, 
            name: "Terbium".to_owned(), 
            symbol:"Tb".to_owned(),
            mass: 158.92535_f32,
            electron_configuration: "[Xe]6s2 4f9".to_owned()
        });
    
        number_to_element.insert(66, Element { 
            number: 66, 
            name: "Dysprosium".to_owned(), 
            symbol:"Dy".to_owned(),
            mass: 162.500_f32,
            electron_configuration: "[Xe]6s2 4f10".to_owned()
        });
    
        number_to_element.insert(67, Element { 
            number: 67, 
            name: "Holmium".to_owned(), 
            symbol:"Ho".to_owned(),
            mass: 164.93033_f32,
            electron_configuration: "[Xe]6s2 4f11".to_owned()
        });
    
        number_to_element.insert(68, Element { 
            number: 68, 
            name: "Erbium".to_owned(), 
            symbol:"Er".to_owned(),
            mass: 167.26_f32,
            electron_configuration: "[Xe]6s2 4f12".to_owned()
        });
    
        number_to_element.insert(69, Element { 
            number: 69, 
            name: "Thulium".to_owned(), 
            symbol:"Tm".to_owned(),
            mass: 168.93422_f32,
            electron_configuration: "[Xe]6s2 4f13".to_owned()
        });
    
        number_to_element.insert(70, Element { 
            number: 70, 
            name: "Ytterbium".to_owned(), 
            symbol:"Yb".to_owned(),
            mass: 173.05_f32,
            electron_configuration: "[Xe]6s2 4f14".to_owned()
        });
    
        number_to_element.insert(71, Element { 
            number: 71, 
            name: "Lutetium".to_owned(), 
            symbol:"Lu".to_owned(),
            mass: 174.9668_f32,
            electron_configuration: "[Xe]6s2 4f14 5d1".to_owned()
        });
    
        number_to_element.insert(72, Element { 
            number: 72, 
            name: "Hafnium".to_owned(), 
            symbol:"Hf".to_owned(),
            mass: 178.49_f32,
            electron_configuration: "[Xe]6s2 4f14 5d2".to_owned()
        });
    
        number_to_element.insert(73, Element { 
            number: 73, 
            name: "Tantalum".to_owned(), 
            symbol:"Ta".to_owned(),
            mass: 180.9479_f32,
            electron_configuration: "[Xe]6s2 4f14 5d3".to_owned()
        });
    
        number_to_element.insert(74, Element { 
            number: 74, 
            name: "Tungsten".to_owned(), 
            symbol:"W".to_owned(),
            mass: 183.84_f32,
            electron_configuration: "[Xe]6s2 4f14 5d4".to_owned()
        });
    
        number_to_element.insert(75, Element { 
            number: 75, 
            name: "Rhenium".to_owned(), 
            symbol:"Re".to_owned(),
            mass: 186.207_f32,
            electron_configuration: "[Xe]6s2 4f14 5d5".to_owned()
        });
    
        number_to_element.insert(76, Element { 
            number: 76, 
            name: "Osmium".to_owned(), 
            symbol:"Os".to_owned(),
            mass: 190.2_f32,
            electron_configuration: "[Xe]6s2 4f14 5d6".to_owned()
        });
    
        number_to_element.insert(77, Element { 
            number: 77, 
            name: "Iridium".to_owned(), 
            symbol:"Ir".to_owned(),
            mass: 192.22_f32,
            electron_configuration: "[Xe]6s2 4f14 5d7".to_owned()
        });
    
        number_to_element.insert(78, Element { 
            number: 78, 
            name: "Platinum".to_owned(), 
            symbol:"Pt".to_owned(),
            mass: 195.08_f32,
            electron_configuration: "[Xe]6s1 4f14 5d9".to_owned()
        });
    
        number_to_element.insert(79, Element { 
            number: 79, 
            name: "Gold".to_owned(), 
            symbol:"Au".to_owned(),
            mass: 196.96657_f32,
            electron_configuration: "[Xe]6s1 4f14 5d10".to_owned()
        });
    
        number_to_element.insert(80, Element { 
            number: 80, 
            name: "Mercury".to_owned(), 
            symbol:"Hg".to_owned(),
            mass: 200.59_f32,
            electron_configuration: "[Xe]6s2 4f14 5d10".to_owned()
        });
    
        number_to_element.insert(81, Element { 
            number: 81, 
            name: "Thallium".to_owned(), 
            symbol:"Tl".to_owned(),
            mass: 204.383_f32,
            electron_configuration: "[Xe]6s2 4f14 5d10 6p1".to_owned()
        });
    
        number_to_element.insert(82, Element { 
            number: 82, 
            name: "Lead".to_owned(), 
            symbol:"Pb".to_owned(),
            mass: 207_f32,
            electron_configuration: "[Xe]6s2 4f14 5d10 6p2".to_owned()
        });
    
        number_to_element.insert(83, Element { 
            number: 83, 
            name: "Bismuth".to_owned(), 
            symbol:"Bi".to_owned(),
            mass: 208.98040_f32,
            electron_configuration: "[Xe]6s2 4f14 5d10 6p3".to_owned()
        });
    
        number_to_element.insert(84, Element { 
            number: 84, 
            name: "Polonium".to_owned(), 
            symbol:"Po".to_owned(),
            mass: 208.98243_f32,
            electron_configuration: "[Xe]6s2 4f14 5d10 6p4".to_owned()
        });
    
        number_to_element.insert(85, Element { 
            number: 85, 
            name: "Astatine".to_owned(), 
            symbol:"At".to_owned(),
            mass: 209.98715_f32,
            electron_configuration: "[Xe]6s2 4f14 5d10 6p5".to_owned()
        });
    
        number_to_element.insert(86, Element { 
            number: 86, 
            name: "Radon".to_owned(), 
            symbol:"Rn".to_owned(),
            mass: 222.01758_f32,
            electron_configuration: "[Xe]6s2 4f14 5d10 6p6".to_owned()
        });
    
        number_to_element.insert(87, Element { 
            number: 87, 
            name: "Francium".to_owned(), 
            symbol:"Fr".to_owned(),
            mass: 223.01973_f32,
            electron_configuration: "[Rn]7s1".to_owned()
        });
    
        number_to_element.insert(88, Element { 
            number: 88, 
            name: "Radium".to_owned(), 
            symbol:"Ra".to_owned(),
            mass: 226.02541_f32,
            electron_configuration: "[Rn]7s2".to_owned()
        });
    
        number_to_element.insert(89, Element { 
            number: 89, 
            name: "Actinium".to_owned(), 
            symbol:"Ac".to_owned(),
            mass: 227.02775_f32,
            electron_configuration: "[Rn]7s2 6d1".to_owned()
        });
    
        number_to_element.insert(90, Element { 
            number: 90, 
            name: "Thorium".to_owned(), 
            symbol:"Th".to_owned(),
            mass: 232.038_f32,
            electron_configuration: "[Rn]7s2 6d2".to_owned()
        });
    
        number_to_element.insert(91, Element { 
            number: 91, 
            name: "Protactinium".to_owned(), 
            symbol:"Pa".to_owned(),
            mass: 231.03588_f32,
            electron_configuration: "[Rn]7s2 5f2 6d1".to_owned()
        });
    
        number_to_element.insert(92, Element { 
            number: 92, 
            name: "Uranium".to_owned(), 
            symbol:"U".to_owned(),
            mass: 238.0289_f32,
            electron_configuration: "[Rn]7s2 5f3 6d1".to_owned()
        });
    
        number_to_element.insert(93, Element { 
            number: 93, 
            name: "Neptunium".to_owned(), 
            symbol:"Np".to_owned(),
            mass: 237.048172_f32,
            electron_configuration: "[Rn]7s2 5f4 6d1".to_owned()
        });
    
        number_to_element.insert(94, Element { 
            number: 94, 
            name: "Plutonium".to_owned(), 
            symbol:"Pu".to_owned(),
            mass: 244.06420_f32,
            electron_configuration: "[Rn]7s2 5f6".to_owned()
        });
    
        number_to_element.insert(95, Element { 
            number: 95, 
            name: "Americium".to_owned(), 
            symbol:"Am".to_owned(),
            mass: 243.061380_f32,
            electron_configuration: "[Rn]7s2 5f7".to_owned()
        });
    
        number_to_element.insert(96, Element { 
            number: 96, 
            name: "Curium".to_owned(), 
            symbol:"Cm".to_owned(),
            mass: 247.07035_f32,
            electron_configuration: "[Rn]7s2 5f7 6d1".to_owned()
        });
    
        number_to_element.insert(97, Element { 
            number: 97, 
            name: "Berkelium".to_owned(), 
            symbol:"Bk".to_owned(),
            mass: 247.07031_f32,
            electron_configuration: "[Rn]7s2 5f9".to_owned()
        });
    
        number_to_element.insert(98, Element { 
            number: 98, 
            name: "Californium".to_owned(), 
            symbol:"Cf".to_owned(),
            mass: 251.07959_f32,
            electron_configuration: "[Rn]7s2 5f10".to_owned()
        });
    
        number_to_element.insert(99, Element { 
            number: 99, 
            name: "Einsteinium".to_owned(), 
            symbol:"Es".to_owned(),
            mass: 252.0830_f32,
            electron_configuration: "[Rn]7s2 5f11".to_owned()
        });
    
        number_to_element.insert(100, Element { 
            number: 100, 
            name: "Fermium".to_owned(), 
            symbol:"Fm".to_owned(),
            mass: 257.09511_f32,
            electron_configuration: "[Rn] 5f12 7s2".to_owned()
        });
    
        number_to_element.insert(101, Element { 
            number: 101, 
            name: "Mendelevium".to_owned(), 
            symbol:"Md".to_owned(),
            mass: 258.09843_f32,
            electron_configuration: "[Rn]7s2 5f13".to_owned()
        });
    
        number_to_element.insert(102, Element { 
            number: 102, 
            name: "Nobelium".to_owned(), 
            symbol:"No".to_owned(),
            mass: 259.10100_f32,
            electron_configuration: "[Rn]7s2 5f14".to_owned()
        });
    
        number_to_element.insert(103, Element { 
            number: 103, 
            name: "Lawrencium".to_owned(), 
            symbol:"Lr".to_owned(),
            mass: 266.120_f32,
            electron_configuration: "[Rn]7s2 5f14 6d1".to_owned()
        });
    
        number_to_element.insert(104, Element { 
            number: 104, 
            name: "Rutherfordium".to_owned(), 
            symbol:"Rf".to_owned(),
            mass: 267.122_f32,
            electron_configuration: "[Rn]7s2 5f14 6d2".to_owned()
        });
    
        number_to_element.insert(105, Element { 
            number: 105, 
            name: "Dubnium".to_owned(), 
            symbol:"Db".to_owned(),
            mass: 268.126_f32,
            electron_configuration: "[Rn]7s2 5f14 6d3".to_owned()
        });
    
        number_to_element.insert(106, Element { 
            number: 106, 
            name: "Seaborgium".to_owned(), 
            symbol:"Sg".to_owned(),
            mass: 269.128_f32,
            electron_configuration: "[Rn]7s2 5f14 6d4".to_owned()
        });
    
        number_to_element.insert(107, Element { 
            number: 107, 
            name: "Bohrium".to_owned(), 
            symbol:"Bh".to_owned(),
            mass: 270.133_f32,
            electron_configuration: "[Rn]7s2 5f14 6d5".to_owned()
        });
    
        number_to_element.insert(108, Element { 
            number: 108, 
            name: "Hassium".to_owned(), 
            symbol:"Hs".to_owned(),
            mass: 269.1336_f32,
            electron_configuration: "[Rn]7s2 5f14 6d6".to_owned()
        });
    
        number_to_element.insert(109, Element { 
            number: 109, 
            name: "Meitnerium".to_owned(), 
            symbol:"Mt".to_owned(),
            mass: 277.154_f32,
            electron_configuration: "[Rn]7s2 5f14 6d7 (calculated)".to_owned()
        });
    
        number_to_element.insert(110, Element { 
            number: 110, 
            name: "Darmstadtium".to_owned(), 
            symbol:"Ds".to_owned(),
            mass: 282.166_f32,
            electron_configuration: "[Rn]7s2 5f14 6d8 (predicted)".to_owned()
        });
    
        number_to_element.insert(111, Element { 
            number: 111, 
            name: "Roentgenium".to_owned(), 
            symbol:"Rg".to_owned(),
            mass: 282.169_f32,
            electron_configuration: "[Rn]7s2 5f14 6d9 (predicted)".to_owned()
        });
    
        number_to_element.insert(112, Element { 
            number: 112, 
            name: "Copernicium".to_owned(), 
            symbol:"Cn".to_owned(),
            mass: 286.179_f32,
            electron_configuration: "[Rn]7s2 5f14 6d10 (predicted)".to_owned()
        });
    
        number_to_element.insert(113, Element { 
            number: 113, 
            name: "Nihonium".to_owned(), 
            symbol:"Nh".to_owned(),
            mass: 286.182_f32,
            electron_configuration: "[Rn]5f14 6d10 7s2 7p1 (predicted)".to_owned()
        });
    
        number_to_element.insert(114, Element { 
            number: 114, 
            name: "Flerovium".to_owned(), 
            symbol:"Fl".to_owned(),
            mass: 290.192_f32,
            electron_configuration: "[Rn]7s2 7p2 5f14 6d10 (predicted)".to_owned()
        });
    
        number_to_element.insert(115, Element { 
            number: 115, 
            name: "Moscovium".to_owned(), 
            symbol:"Mc".to_owned(),
            mass: 290.196_f32,
            electron_configuration: "[Rn]7s2 7p3 5f14 6d10 (predicted)".to_owned()
        });
    
        number_to_element.insert(116, Element { 
            number: 116, 
            name: "Livermorium".to_owned(), 
            symbol:"Lv".to_owned(),
            mass: 293.205_f32,
            electron_configuration: "[Rn]7s2 7p4 5f14 6d10 (predicted)".to_owned()
        });
    
        number_to_element.insert(117, Element { 
            number: 117, 
            name: "Tennessine".to_owned(), 
            symbol:"Ts".to_owned(),
            mass: 294.211_f32,
            electron_configuration: "[Rn]7s2 7p5 5f14 6d10 (predicted)".to_owned()
        });
    
        number_to_element.insert(118, Element { 
            number: 118, 
            name: "Oganesson".to_owned(), 
            symbol:"Og".to_owned(),
            mass: 295.216_f32,
            electron_configuration: "[Rn]7s2 7p6 5f14 6d10 (predicted)".to_owned()
        });
    
    // INPUT TO ELEMENT - match Identifier to Element (proc macro this)
    let output: &Element = match arg.as_str() {"H" => &number_to_element[1],
            "Hydrogen" => &number_to_element[1],
            "1" => &number_to_element[1],
            "He" => &number_to_element[2],
            "Helium" => &number_to_element[2],
            "2" => &number_to_element[2],
            "Li" => &number_to_element[3],
            "Lithium" => &number_to_element[3],
            "3" => &number_to_element[3],
            "Be" => &number_to_element[4],
            "Beryllium" => &number_to_element[4],
            "4" => &number_to_element[4],
            "B" => &number_to_element[5],
            "Boron" => &number_to_element[5],
            "5" => &number_to_element[5],
            "C" => &number_to_element[6],
            "Carbon" => &number_to_element[6],
            "6" => &number_to_element[6],
            "N" => &number_to_element[7],
            "Nitrogen" => &number_to_element[7],
            "7" => &number_to_element[7],
            "O" => &number_to_element[8],
            "Oxygen" => &number_to_element[8],
            "8" => &number_to_element[8],
            "F" => &number_to_element[9],
            "Fluorine" => &number_to_element[9],
            "9" => &number_to_element[9],
            "Ne" => &number_to_element[10],
            "Neon" => &number_to_element[10],
            "10" => &number_to_element[10],
            "Na" => &number_to_element[11],
            "Sodium" => &number_to_element[11],
            "11" => &number_to_element[11],
            "Mg" => &number_to_element[12],
            "Magnesium" => &number_to_element[12],
            "12" => &number_to_element[12],
            "Al" => &number_to_element[13],
            "Aluminum" => &number_to_element[13],
            "13" => &number_to_element[13],
            "Si" => &number_to_element[14],
            "Silicon" => &number_to_element[14],
            "14" => &number_to_element[14],
            "P" => &number_to_element[15],
            "Phosphorus" => &number_to_element[15],
            "15" => &number_to_element[15],
            "S" => &number_to_element[16],
            "Sulfur" => &number_to_element[16],
            "16" => &number_to_element[16],
            "Cl" => &number_to_element[17],
            "Chlorine" => &number_to_element[17],
            "17" => &number_to_element[17],
            "Ar" => &number_to_element[18],
            "Argon" => &number_to_element[18],
            "18" => &number_to_element[18],
            "K" => &number_to_element[19],
            "Potassium" => &number_to_element[19],
            "19" => &number_to_element[19],
            "Ca" => &number_to_element[20],
            "Calcium" => &number_to_element[20],
            "20" => &number_to_element[20],
            "Sc" => &number_to_element[21],
            "Scandium" => &number_to_element[21],
            "21" => &number_to_element[21],
            "Ti" => &number_to_element[22],
            "Titanium" => &number_to_element[22],
            "22" => &number_to_element[22],
            "V" => &number_to_element[23],
            "Vanadium" => &number_to_element[23],
            "23" => &number_to_element[23],
            "Cr" => &number_to_element[24],
            "Chromium" => &number_to_element[24],
            "24" => &number_to_element[24],
            "Mn" => &number_to_element[25],
            "Manganese" => &number_to_element[25],
            "25" => &number_to_element[25],
            "Fe" => &number_to_element[26],
            "Iron" => &number_to_element[26],
            "26" => &number_to_element[26],
            "Co" => &number_to_element[27],
            "Cobalt" => &number_to_element[27],
            "27" => &number_to_element[27],
            "Ni" => &number_to_element[28],
            "Nickel" => &number_to_element[28],
            "28" => &number_to_element[28],
            "Cu" => &number_to_element[29],
            "Copper" => &number_to_element[29],
            "29" => &number_to_element[29],
            "Zn" => &number_to_element[30],
            "Zinc" => &number_to_element[30],
            "30" => &number_to_element[30],
            "Ga" => &number_to_element[31],
            "Gallium" => &number_to_element[31],
            "31" => &number_to_element[31],
            "Ge" => &number_to_element[32],
            "Germanium" => &number_to_element[32],
            "32" => &number_to_element[32],
            "As" => &number_to_element[33],
            "Arsenic" => &number_to_element[33],
            "33" => &number_to_element[33],
            "Se" => &number_to_element[34],
            "Selenium" => &number_to_element[34],
            "34" => &number_to_element[34],
            "Br" => &number_to_element[35],
            "Bromine" => &number_to_element[35],
            "35" => &number_to_element[35],
            "Kr" => &number_to_element[36],
            "Krypton" => &number_to_element[36],
            "36" => &number_to_element[36],
            "Rb" => &number_to_element[37],
            "Rubidium" => &number_to_element[37],
            "37" => &number_to_element[37],
            "Sr" => &number_to_element[38],
            "Strontium" => &number_to_element[38],
            "38" => &number_to_element[38],
            "Y" => &number_to_element[39],
            "Yttrium" => &number_to_element[39],
            "39" => &number_to_element[39],
            "Zr" => &number_to_element[40],
            "Zirconium" => &number_to_element[40],
            "40" => &number_to_element[40],
            "Nb" => &number_to_element[41],
            "Niobium" => &number_to_element[41],
            "41" => &number_to_element[41],
            "Mo" => &number_to_element[42],
            "Molybdenum" => &number_to_element[42],
            "42" => &number_to_element[42],
            "Tc" => &number_to_element[43],
            "Technetium" => &number_to_element[43],
            "43" => &number_to_element[43],
            "Ru" => &number_to_element[44],
            "Ruthenium" => &number_to_element[44],
            "44" => &number_to_element[44],
            "Rh" => &number_to_element[45],
            "Rhodium" => &number_to_element[45],
            "45" => &number_to_element[45],
            "Pd" => &number_to_element[46],
            "Palladium" => &number_to_element[46],
            "46" => &number_to_element[46],
            "Ag" => &number_to_element[47],
            "Silver" => &number_to_element[47],
            "47" => &number_to_element[47],
            "Cd" => &number_to_element[48],
            "Cadmium" => &number_to_element[48],
            "48" => &number_to_element[48],
            "In" => &number_to_element[49],
            "Indium" => &number_to_element[49],
            "49" => &number_to_element[49],
            "Sn" => &number_to_element[50],
            "Tin" => &number_to_element[50],
            "50" => &number_to_element[50],
            "Sb" => &number_to_element[51],
            "Antimony" => &number_to_element[51],
            "51" => &number_to_element[51],
            "Te" => &number_to_element[52],
            "Tellurium" => &number_to_element[52],
            "52" => &number_to_element[52],
            "I" => &number_to_element[53],
            "Iodine" => &number_to_element[53],
            "53" => &number_to_element[53],
            "Xe" => &number_to_element[54],
            "Xenon" => &number_to_element[54],
            "54" => &number_to_element[54],
            "Cs" => &number_to_element[55],
            "Cesium" => &number_to_element[55],
            "55" => &number_to_element[55],
            "Ba" => &number_to_element[56],
            "Barium" => &number_to_element[56],
            "56" => &number_to_element[56],
            "La" => &number_to_element[57],
            "Lanthanum" => &number_to_element[57],
            "57" => &number_to_element[57],
            "Ce" => &number_to_element[58],
            "Cerium" => &number_to_element[58],
            "58" => &number_to_element[58],
            "Pr" => &number_to_element[59],
            "Praseodymium" => &number_to_element[59],
            "59" => &number_to_element[59],
            "Nd" => &number_to_element[60],
            "Neodymium" => &number_to_element[60],
            "60" => &number_to_element[60],
            "Pm" => &number_to_element[61],
            "Promethium" => &number_to_element[61],
            "61" => &number_to_element[61],
            "Sm" => &number_to_element[62],
            "Samarium" => &number_to_element[62],
            "62" => &number_to_element[62],
            "Eu" => &number_to_element[63],
            "Europium" => &number_to_element[63],
            "63" => &number_to_element[63],
            "Gd" => &number_to_element[64],
            "Gadolinium" => &number_to_element[64],
            "64" => &number_to_element[64],
            "Tb" => &number_to_element[65],
            "Terbium" => &number_to_element[65],
            "65" => &number_to_element[65],
            "Dy" => &number_to_element[66],
            "Dysprosium" => &number_to_element[66],
            "66" => &number_to_element[66],
            "Ho" => &number_to_element[67],
            "Holmium" => &number_to_element[67],
            "67" => &number_to_element[67],
            "Er" => &number_to_element[68],
            "Erbium" => &number_to_element[68],
            "68" => &number_to_element[68],
            "Tm" => &number_to_element[69],
            "Thulium" => &number_to_element[69],
            "69" => &number_to_element[69],
            "Yb" => &number_to_element[70],
            "Ytterbium" => &number_to_element[70],
            "70" => &number_to_element[70],
            "Lu" => &number_to_element[71],
            "Lutetium" => &number_to_element[71],
            "71" => &number_to_element[71],
            "Hf" => &number_to_element[72],
            "Hafnium" => &number_to_element[72],
            "72" => &number_to_element[72],
            "Ta" => &number_to_element[73],
            "Tantalum" => &number_to_element[73],
            "73" => &number_to_element[73],
            "W" => &number_to_element[74],
            "Tungsten" => &number_to_element[74],
            "74" => &number_to_element[74],
            "Re" => &number_to_element[75],
            "Rhenium" => &number_to_element[75],
            "75" => &number_to_element[75],
            "Os" => &number_to_element[76],
            "Osmium" => &number_to_element[76],
            "76" => &number_to_element[76],
            "Ir" => &number_to_element[77],
            "Iridium" => &number_to_element[77],
            "77" => &number_to_element[77],
            "Pt" => &number_to_element[78],
            "Platinum" => &number_to_element[78],
            "78" => &number_to_element[78],
            "Au" => &number_to_element[79],
            "Gold" => &number_to_element[79],
            "79" => &number_to_element[79],
            "Hg" => &number_to_element[80],
            "Mercury" => &number_to_element[80],
            "80" => &number_to_element[80],
            "Tl" => &number_to_element[81],
            "Thallium" => &number_to_element[81],
            "81" => &number_to_element[81],
            "Pb" => &number_to_element[82],
            "Lead" => &number_to_element[82],
            "82" => &number_to_element[82],
            "Bi" => &number_to_element[83],
            "Bismuth" => &number_to_element[83],
            "83" => &number_to_element[83],
            "Po" => &number_to_element[84],
            "Polonium" => &number_to_element[84],
            "84" => &number_to_element[84],
            "At" => &number_to_element[85],
            "Astatine" => &number_to_element[85],
            "85" => &number_to_element[85],
            "Rn" => &number_to_element[86],
            "Radon" => &number_to_element[86],
            "86" => &number_to_element[86],
            "Fr" => &number_to_element[87],
            "Francium" => &number_to_element[87],
            "87" => &number_to_element[87],
            "Ra" => &number_to_element[88],
            "Radium" => &number_to_element[88],
            "88" => &number_to_element[88],
            "Ac" => &number_to_element[89],
            "Actinium" => &number_to_element[89],
            "89" => &number_to_element[89],
            "Th" => &number_to_element[90],
            "Thorium" => &number_to_element[90],
            "90" => &number_to_element[90],
            "Pa" => &number_to_element[91],
            "Protactinium" => &number_to_element[91],
            "91" => &number_to_element[91],
            "U" => &number_to_element[92],
            "Uranium" => &number_to_element[92],
            "92" => &number_to_element[92],
            "Np" => &number_to_element[93],
            "Neptunium" => &number_to_element[93],
            "93" => &number_to_element[93],
            "Pu" => &number_to_element[94],
            "Plutonium" => &number_to_element[94],
            "94" => &number_to_element[94],
            "Am" => &number_to_element[95],
            "Americium" => &number_to_element[95],
            "95" => &number_to_element[95],
            "Cm" => &number_to_element[96],
            "Curium" => &number_to_element[96],
            "96" => &number_to_element[96],
            "Bk" => &number_to_element[97],
            "Berkelium" => &number_to_element[97],
            "97" => &number_to_element[97],
            "Cf" => &number_to_element[98],
            "Californium" => &number_to_element[98],
            "98" => &number_to_element[98],
            "Es" => &number_to_element[99],
            "Einsteinium" => &number_to_element[99],
            "99" => &number_to_element[99],
            "Fm" => &number_to_element[100],
            "Fermium" => &number_to_element[100],
            "100" => &number_to_element[100],
            "Md" => &number_to_element[101],
            "Mendelevium" => &number_to_element[101],
            "101" => &number_to_element[101],
            "No" => &number_to_element[102],
            "Nobelium" => &number_to_element[102],
            "102" => &number_to_element[102],
            "Lr" => &number_to_element[103],
            "Lawrencium" => &number_to_element[103],
            "103" => &number_to_element[103],
            "Rf" => &number_to_element[104],
            "Rutherfordium" => &number_to_element[104],
            "104" => &number_to_element[104],
            "Db" => &number_to_element[105],
            "Dubnium" => &number_to_element[105],
            "105" => &number_to_element[105],
            "Sg" => &number_to_element[106],
            "Seaborgium" => &number_to_element[106],
            "106" => &number_to_element[106],
            "Bh" => &number_to_element[107],
            "Bohrium" => &number_to_element[107],
            "107" => &number_to_element[107],
            "Hs" => &number_to_element[108],
            "Hassium" => &number_to_element[108],
            "108" => &number_to_element[108],
            "Mt" => &number_to_element[109],
            "Meitnerium" => &number_to_element[109],
            "109" => &number_to_element[109],
            "Ds" => &number_to_element[110],
            "Darmstadtium" => &number_to_element[110],
            "110" => &number_to_element[110],
            "Rg" => &number_to_element[111],
            "Roentgenium" => &number_to_element[111],
            "111" => &number_to_element[111],
            "Cn" => &number_to_element[112],
            "Copernicium" => &number_to_element[112],
            "112" => &number_to_element[112],
            "Nh" => &number_to_element[113],
            "Nihonium" => &number_to_element[113],
            "113" => &number_to_element[113],
            "Fl" => &number_to_element[114],
            "Flerovium" => &number_to_element[114],
            "114" => &number_to_element[114],
            "Mc" => &number_to_element[115],
            "Moscovium" => &number_to_element[115],
            "115" => &number_to_element[115],
            "Lv" => &number_to_element[116],
            "Livermorium" => &number_to_element[116],
            "116" => &number_to_element[116],
            "Ts" => &number_to_element[117],
            "Tennessine" => &number_to_element[117],
            "117" => &number_to_element[117],
            "Og" => &number_to_element[118],
            "Oganesson" => &number_to_element[118],
            "118" => &number_to_element[118],
            _ => panic!("Invalid element identifier."),
    };

    println!("{}", output);

}