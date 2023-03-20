#![allow(clippy::excessive_precision)]

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

        tmp.insert(1, Element { 
            number: 1, 
            name: "Hydrogen".to_owned(), 
            symbol: "H".to_owned(),
            mass: 1.0080_f32,
            electron_configuration: "1s1".to_owned()
        });

        tmp.insert(2, Element { 
            number: 2, 
            name: "Helium".to_owned(), 
            symbol: "He".to_owned(),
            mass: 4.00260_f32,
            electron_configuration: "1s2".to_owned()
        });

        tmp.insert(3, Element { 
            number: 3, 
            name: "Lithium".to_owned(), 
            symbol: "Li".to_owned(),
            mass: 7.0_f32,
            electron_configuration: "[He]2s1".to_owned()
        });

        tmp.insert(4, Element { 
            number: 4, 
            name: "Beryllium".to_owned(), 
            symbol: "Be".to_owned(),
            mass: 9.012183_f32,
            electron_configuration: "[He]2s2".to_owned()
        });

        tmp.insert(5, Element { 
            number: 5, 
            name: "Boron".to_owned(), 
            symbol: "B".to_owned(),
            mass: 10.81_f32,
            electron_configuration: "[He]2s2 2p1".to_owned()
        });

        tmp.insert(6, Element { 
            number: 6, 
            name: "Carbon".to_owned(), 
            symbol: "C".to_owned(),
            mass: 12.011_f32,
            electron_configuration: "[He]2s2 2p2".to_owned()
        });

        tmp.insert(7, Element { 
            number: 7, 
            name: "Nitrogen".to_owned(), 
            symbol: "N".to_owned(),
            mass: 14.007_f32,
            electron_configuration: "[He] 2s2 2p3".to_owned()
        });

        tmp.insert(8, Element { 
            number: 8, 
            name: "Oxygen".to_owned(), 
            symbol: "O".to_owned(),
            mass: 15.999_f32,
            electron_configuration: "[He]2s2 2p4".to_owned()
        });

        tmp.insert(9, Element { 
            number: 9, 
            name: "Fluorine".to_owned(), 
            symbol: "F".to_owned(),
            mass: 18.99840316_f32,
            electron_configuration: "[He]2s2 2p5".to_owned()
        });

        tmp.insert(10, Element { 
            number: 10, 
            name: "Neon".to_owned(), 
            symbol: "Ne".to_owned(),
            mass: 20.180_f32,
            electron_configuration: "[He]2s2 2p6".to_owned()
        });

        tmp.insert(11, Element { 
            number: 11, 
            name: "Sodium".to_owned(), 
            symbol: "Na".to_owned(),
            mass: 22.9897693_f32,
            electron_configuration: "[Ne]3s1".to_owned()
        });

        tmp.insert(12, Element { 
            number: 12, 
            name: "Magnesium".to_owned(), 
            symbol: "Mg".to_owned(),
            mass: 24.305_f32,
            electron_configuration: "[Ne]3s2".to_owned()
        });

        tmp.insert(13, Element { 
            number: 13, 
            name: "Aluminum".to_owned(), 
            symbol: "Al".to_owned(),
            mass: 26.981538_f32,
            electron_configuration: "[Ne]3s2 3p1".to_owned()
        });

        tmp.insert(14, Element { 
            number: 14, 
            name: "Silicon".to_owned(), 
            symbol: "Si".to_owned(),
            mass: 28.085_f32,
            electron_configuration: "[Ne]3s2 3p2".to_owned()
        });

        tmp.insert(15, Element { 
            number: 15, 
            name: "Phosphorus".to_owned(), 
            symbol: "P".to_owned(),
            mass: 30.97376200_f32,
            electron_configuration: "[Ne]3s2 3p3".to_owned()
        });

        tmp.insert(16, Element { 
            number: 16, 
            name: "Sulfur".to_owned(), 
            symbol: "S".to_owned(),
            mass: 32.07_f32,
            electron_configuration: "[Ne]3s2 3p4".to_owned()
        });

        tmp.insert(17, Element { 
            number: 17, 
            name: "Chlorine".to_owned(), 
            symbol: "Cl".to_owned(),
            mass: 35.45_f32,
            electron_configuration: "[Ne]3s2 3p5".to_owned()
        });

        tmp.insert(18, Element { 
            number: 18, 
            name: "Argon".to_owned(), 
            symbol: "Ar".to_owned(),
            mass: 39.9_f32,
            electron_configuration: "[Ne]3s2 3p6".to_owned()
        });

        tmp.insert(19, Element { 
            number: 19, 
            name: "Potassium".to_owned(), 
            symbol: "K".to_owned(),
            mass: 39.0983_f32,
            electron_configuration: "[Ar]4s1".to_owned()
        });

        tmp.insert(20, Element { 
            number: 20, 
            name: "Calcium".to_owned(), 
            symbol: "Ca".to_owned(),
            mass: 40.08_f32,
            electron_configuration: "[Ar]4s2".to_owned()
        });

        tmp.insert(21, Element { 
            number: 21, 
            name: "Scandium".to_owned(), 
            symbol: "Sc".to_owned(),
            mass: 44.95591_f32,
            electron_configuration: "[Ar]4s2 3d1".to_owned()
        });

        tmp.insert(22, Element { 
            number: 22, 
            name: "Titanium".to_owned(), 
            symbol: "Ti".to_owned(),
            mass: 47.867_f32,
            electron_configuration: "[Ar]4s2 3d2".to_owned()
        });

        tmp.insert(23, Element { 
            number: 23, 
            name: "Vanadium".to_owned(), 
            symbol: "V".to_owned(),
            mass: 50.9415_f32,
            electron_configuration: "[Ar]4s2 3d3".to_owned()
        });

        tmp.insert(24, Element { 
            number: 24, 
            name: "Chromium".to_owned(), 
            symbol: "Cr".to_owned(),
            mass: 51.996_f32,
            electron_configuration: "[Ar]3d5 4s1".to_owned()
        });

        tmp.insert(25, Element { 
            number: 25, 
            name: "Manganese".to_owned(), 
            symbol: "Mn".to_owned(),
            mass: 54.93804_f32,
            electron_configuration: "[Ar]4s2 3d5".to_owned()
        });

        tmp.insert(26, Element { 
            number: 26, 
            name: "Iron".to_owned(), 
            symbol: "Fe".to_owned(),
            mass: 55.84_f32,
            electron_configuration: "[Ar]4s2 3d6".to_owned()
        });

        tmp.insert(27, Element { 
            number: 27, 
            name: "Cobalt".to_owned(), 
            symbol: "Co".to_owned(),
            mass: 58.93319_f32,
            electron_configuration: "[Ar]4s2 3d7".to_owned()
        });

        tmp.insert(28, Element { 
            number: 28, 
            name: "Nickel".to_owned(), 
            symbol: "Ni".to_owned(),
            mass: 58.693_f32,
            electron_configuration: "[Ar]4s2 3d8".to_owned()
        });

        tmp.insert(29, Element { 
            number: 29, 
            name: "Copper".to_owned(), 
            symbol: "Cu".to_owned(),
            mass: 63.55_f32,
            electron_configuration: "[Ar]4s1 3d10".to_owned()
        });

        tmp.insert(30, Element { 
            number: 30, 
            name: "Zinc".to_owned(), 
            symbol: "Zn".to_owned(),
            mass: 65.4_f32,
            electron_configuration: "[Ar]4s2 3d10".to_owned()
        });

        tmp.insert(31, Element { 
            number: 31, 
            name: "Gallium".to_owned(), 
            symbol: "Ga".to_owned(),
            mass: 69.723_f32,
            electron_configuration: "[Ar]4s2 3d10 4p1".to_owned()
        });

        tmp.insert(32, Element { 
            number: 32, 
            name: "Germanium".to_owned(), 
            symbol: "Ge".to_owned(),
            mass: 72.63_f32,
            electron_configuration: "[Ar]4s2 3d10 4p2".to_owned()
        });

        tmp.insert(33, Element { 
            number: 33, 
            name: "Arsenic".to_owned(), 
            symbol: "As".to_owned(),
            mass: 74.92159_f32,
            electron_configuration: "[Ar]4s2 3d10 4p3".to_owned()
        });

        tmp.insert(34, Element { 
            number: 34, 
            name: "Selenium".to_owned(), 
            symbol: "Se".to_owned(),
            mass: 78.97_f32,
            electron_configuration: "[Ar]4s2 3d10 4p4".to_owned()
        });

        tmp.insert(35, Element { 
            number: 35, 
            name: "Bromine".to_owned(), 
            symbol: "Br".to_owned(),
            mass: 79.90_f32,
            electron_configuration: "[Ar]4s2 3d10 4p5".to_owned()
        });

        tmp.insert(36, Element { 
            number: 36, 
            name: "Krypton".to_owned(), 
            symbol: "Kr".to_owned(),
            mass: 83.80_f32,
            electron_configuration: "[Ar]4s2 3d10 4p6".to_owned()
        });

        tmp.insert(37, Element { 
            number: 37, 
            name: "Rubidium".to_owned(), 
            symbol: "Rb".to_owned(),
            mass: 85.468_f32,
            electron_configuration: "[Kr]5s1".to_owned()
        });

        tmp.insert(38, Element { 
            number: 38, 
            name: "Strontium".to_owned(), 
            symbol: "Sr".to_owned(),
            mass: 87.62_f32,
            electron_configuration: "[Kr]5s2".to_owned()
        });

        tmp.insert(39, Element { 
            number: 39, 
            name: "Yttrium".to_owned(), 
            symbol: "Y".to_owned(),
            mass: 88.90584_f32,
            electron_configuration: "[Kr]5s2 4d1".to_owned()
        });

        tmp.insert(40, Element { 
            number: 40, 
            name: "Zirconium".to_owned(), 
            symbol: "Zr".to_owned(),
            mass: 91.22_f32,
            electron_configuration: "[Kr]5s2 4d2".to_owned()
        });

        tmp.insert(41, Element { 
            number: 41, 
            name: "Niobium".to_owned(), 
            symbol: "Nb".to_owned(),
            mass: 92.90637_f32,
            electron_configuration: "[Kr]5s1 4d4".to_owned()
        });

        tmp.insert(42, Element { 
            number: 42, 
            name: "Molybdenum".to_owned(), 
            symbol: "Mo".to_owned(),
            mass: 95.95_f32,
            electron_configuration: "[Kr]5s1 4d5".to_owned()
        });

        tmp.insert(43, Element { 
            number: 43, 
            name: "Technetium".to_owned(), 
            symbol: "Tc".to_owned(),
            mass: 96.90636_f32,
            electron_configuration: "[Kr]5s2 4d5".to_owned()
        });

        tmp.insert(44, Element { 
            number: 44, 
            name: "Ruthenium".to_owned(), 
            symbol: "Ru".to_owned(),
            mass: 101.1_f32,
            electron_configuration: "[Kr]5s1 4d7".to_owned()
        });

        tmp.insert(45, Element { 
            number: 45, 
            name: "Rhodium".to_owned(), 
            symbol: "Rh".to_owned(),
            mass: 102.9055_f32,
            electron_configuration: "[Kr]5s1 4d8".to_owned()
        });

        tmp.insert(46, Element { 
            number: 46, 
            name: "Palladium".to_owned(), 
            symbol: "Pd".to_owned(),
            mass: 106.42_f32,
            electron_configuration: "[Kr]4d10".to_owned()
        });

        tmp.insert(47, Element { 
            number: 47, 
            name: "Silver".to_owned(), 
            symbol: "Ag".to_owned(),
            mass: 107.868_f32,
            electron_configuration: "[Kr]5s1 4d10".to_owned()
        });

        tmp.insert(48, Element { 
            number: 48, 
            name: "Cadmium".to_owned(), 
            symbol: "Cd".to_owned(),
            mass: 112.41_f32,
            electron_configuration: "[Kr]5s2 4d10".to_owned()
        });

        tmp.insert(49, Element { 
            number: 49, 
            name: "Indium".to_owned(), 
            symbol: "In".to_owned(),
            mass: 114.818_f32,
            electron_configuration: "[Kr]5s2 4d10 5p1".to_owned()
        });

        tmp.insert(50, Element { 
            number: 50, 
            name: "Tin".to_owned(), 
            symbol: "Sn".to_owned(),
            mass: 118.71_f32,
            electron_configuration: "[Kr]5s2 4d10 5p2".to_owned()
        });

        tmp.insert(51, Element { 
            number: 51, 
            name: "Antimony".to_owned(), 
            symbol: "Sb".to_owned(),
            mass: 121.760_f32,
            electron_configuration: "[Kr]5s2 4d10 5p3".to_owned()
        });

        tmp.insert(52, Element { 
            number: 52, 
            name: "Tellurium".to_owned(), 
            symbol: "Te".to_owned(),
            mass: 127.6_f32,
            electron_configuration: "[Kr]5s2 4d10 5p4".to_owned()
        });

        tmp.insert(53, Element { 
            number: 53, 
            name: "Iodine".to_owned(), 
            symbol: "I".to_owned(),
            mass: 126.9045_f32,
            electron_configuration: "[Kr]5s2 4d10 5p5".to_owned()
        });

        tmp.insert(54, Element { 
            number: 54, 
            name: "Xenon".to_owned(), 
            symbol: "Xe".to_owned(),
            mass: 131.29_f32,
            electron_configuration: "[Kr]5s2 4d10 5p6".to_owned()
        });

        tmp.insert(55, Element { 
            number: 55, 
            name: "Cesium".to_owned(), 
            symbol: "Cs".to_owned(),
            mass: 132.9054520_f32,
            electron_configuration: "[Xe]6s1".to_owned()
        });

        tmp.insert(56, Element { 
            number: 56, 
            name: "Barium".to_owned(), 
            symbol: "Ba".to_owned(),
            mass: 137.33_f32,
            electron_configuration: "[Xe]6s2".to_owned()
        });

        tmp.insert(57, Element { 
            number: 57, 
            name: "Lanthanum".to_owned(), 
            symbol: "La".to_owned(),
            mass: 138.9055_f32,
            electron_configuration: "[Xe]6s2 5d1".to_owned()
        });

        tmp.insert(58, Element { 
            number: 58, 
            name: "Cerium".to_owned(), 
            symbol: "Ce".to_owned(),
            mass: 140.116_f32,
            electron_configuration: "[Xe]6s2 4f1 5d1".to_owned()
        });

        tmp.insert(59, Element { 
            number: 59, 
            name: "Praseodymium".to_owned(), 
            symbol: "Pr".to_owned(),
            mass: 140.90766_f32,
            electron_configuration: "[Xe]6s2 4f3".to_owned()
        });

        tmp.insert(60, Element { 
            number: 60, 
            name: "Neodymium".to_owned(), 
            symbol: "Nd".to_owned(),
            mass: 144.24_f32,
            electron_configuration: "[Xe]6s2 4f4".to_owned()
        });

        tmp.insert(61, Element { 
            number: 61, 
            name: "Promethium".to_owned(), 
            symbol: "Pm".to_owned(),
            mass: 144.91276_f32,
            electron_configuration: "[Xe]6s2 4f5".to_owned()
        });

        tmp.insert(62, Element { 
            number: 62, 
            name: "Samarium".to_owned(), 
            symbol: "Sm".to_owned(),
            mass: 150.4_f32,
            electron_configuration: "[Xe]6s2 4f6".to_owned()
        });

        tmp.insert(63, Element { 
            number: 63, 
            name: "Europium".to_owned(), 
            symbol: "Eu".to_owned(),
            mass: 151.964_f32,
            electron_configuration: "[Xe]6s2 4f7".to_owned()
        });

        tmp.insert(64, Element { 
            number: 64, 
            name: "Gadolinium".to_owned(), 
            symbol: "Gd".to_owned(),
            mass: 157.2_f32,
            electron_configuration: "[Xe]6s2 4f7 5d1".to_owned()
        });

        tmp.insert(65, Element { 
            number: 65, 
            name: "Terbium".to_owned(), 
            symbol: "Tb".to_owned(),
            mass: 158.92535_f32,
            electron_configuration: "[Xe]6s2 4f9".to_owned()
        });

        tmp.insert(66, Element { 
            number: 66, 
            name: "Dysprosium".to_owned(), 
            symbol: "Dy".to_owned(),
            mass: 162.500_f32,
            electron_configuration: "[Xe]6s2 4f10".to_owned()
        });

        tmp.insert(67, Element { 
            number: 67, 
            name: "Holmium".to_owned(), 
            symbol: "Ho".to_owned(),
            mass: 164.93033_f32,
            electron_configuration: "[Xe]6s2 4f11".to_owned()
        });

        tmp.insert(68, Element { 
            number: 68, 
            name: "Erbium".to_owned(), 
            symbol: "Er".to_owned(),
            mass: 167.26_f32,
            electron_configuration: "[Xe]6s2 4f12".to_owned()
        });

        tmp.insert(69, Element { 
            number: 69, 
            name: "Thulium".to_owned(), 
            symbol: "Tm".to_owned(),
            mass: 168.93422_f32,
            electron_configuration: "[Xe]6s2 4f13".to_owned()
        });

        tmp.insert(70, Element { 
            number: 70, 
            name: "Ytterbium".to_owned(), 
            symbol: "Yb".to_owned(),
            mass: 173.05_f32,
            electron_configuration: "[Xe]6s2 4f14".to_owned()
        });

        tmp.insert(71, Element { 
            number: 71, 
            name: "Lutetium".to_owned(), 
            symbol: "Lu".to_owned(),
            mass: 174.9668_f32,
            electron_configuration: "[Xe]6s2 4f14 5d1".to_owned()
        });

        tmp.insert(72, Element { 
            number: 72, 
            name: "Hafnium".to_owned(), 
            symbol: "Hf".to_owned(),
            mass: 178.49_f32,
            electron_configuration: "[Xe]6s2 4f14 5d2".to_owned()
        });

        tmp.insert(73, Element { 
            number: 73, 
            name: "Tantalum".to_owned(), 
            symbol: "Ta".to_owned(),
            mass: 180.9479_f32,
            electron_configuration: "[Xe]6s2 4f14 5d3".to_owned()
        });

        tmp.insert(74, Element { 
            number: 74, 
            name: "Tungsten".to_owned(), 
            symbol: "W".to_owned(),
            mass: 183.84_f32,
            electron_configuration: "[Xe]6s2 4f14 5d4".to_owned()
        });

        tmp.insert(75, Element { 
            number: 75, 
            name: "Rhenium".to_owned(), 
            symbol: "Re".to_owned(),
            mass: 186.207_f32,
            electron_configuration: "[Xe]6s2 4f14 5d5".to_owned()
        });

        tmp.insert(76, Element { 
            number: 76, 
            name: "Osmium".to_owned(), 
            symbol: "Os".to_owned(),
            mass: 190.2_f32,
            electron_configuration: "[Xe]6s2 4f14 5d6".to_owned()
        });

        tmp.insert(77, Element { 
            number: 77, 
            name: "Iridium".to_owned(), 
            symbol: "Ir".to_owned(),
            mass: 192.22_f32,
            electron_configuration: "[Xe]6s2 4f14 5d7".to_owned()
        });

        tmp.insert(78, Element { 
            number: 78, 
            name: "Platinum".to_owned(), 
            symbol: "Pt".to_owned(),
            mass: 195.08_f32,
            electron_configuration: "[Xe]6s1 4f14 5d9".to_owned()
        });

        tmp.insert(79, Element { 
            number: 79, 
            name: "Gold".to_owned(), 
            symbol: "Au".to_owned(),
            mass: 196.96657_f32,
            electron_configuration: "[Xe]6s1 4f14 5d10".to_owned()
        });

        tmp.insert(80, Element { 
            number: 80, 
            name: "Mercury".to_owned(), 
            symbol: "Hg".to_owned(),
            mass: 200.59_f32,
            electron_configuration: "[Xe]6s2 4f14 5d10".to_owned()
        });

        tmp.insert(81, Element { 
            number: 81, 
            name: "Thallium".to_owned(), 
            symbol: "Tl".to_owned(),
            mass: 204.383_f32,
            electron_configuration: "[Xe]6s2 4f14 5d10 6p1".to_owned()
        });

        tmp.insert(82, Element { 
            number: 82, 
            name: "Lead".to_owned(), 
            symbol: "Pb".to_owned(),
            mass: 207_f32,
            electron_configuration: "[Xe]6s2 4f14 5d10 6p2".to_owned()
        });

        tmp.insert(83, Element { 
            number: 83, 
            name: "Bismuth".to_owned(), 
            symbol: "Bi".to_owned(),
            mass: 208.98040_f32,
            electron_configuration: "[Xe]6s2 4f14 5d10 6p3".to_owned()
        });

        tmp.insert(84, Element { 
            number: 84, 
            name: "Polonium".to_owned(), 
            symbol: "Po".to_owned(),
            mass: 208.98243_f32,
            electron_configuration: "[Xe]6s2 4f14 5d10 6p4".to_owned()
        });

        tmp.insert(85, Element { 
            number: 85, 
            name: "Astatine".to_owned(), 
            symbol: "At".to_owned(),
            mass: 209.98715_f32,
            electron_configuration: "[Xe]6s2 4f14 5d10 6p5".to_owned()
        });

        tmp.insert(86, Element { 
            number: 86, 
            name: "Radon".to_owned(), 
            symbol: "Rn".to_owned(),
            mass: 222.01758_f32,
            electron_configuration: "[Xe]6s2 4f14 5d10 6p6".to_owned()
        });

        tmp.insert(87, Element { 
            number: 87, 
            name: "Francium".to_owned(), 
            symbol: "Fr".to_owned(),
            mass: 223.01973_f32,
            electron_configuration: "[Rn]7s1".to_owned()
        });

        tmp.insert(88, Element { 
            number: 88, 
            name: "Radium".to_owned(), 
            symbol: "Ra".to_owned(),
            mass: 226.02541_f32,
            electron_configuration: "[Rn]7s2".to_owned()
        });

        tmp.insert(89, Element { 
            number: 89, 
            name: "Actinium".to_owned(), 
            symbol: "Ac".to_owned(),
            mass: 227.02775_f32,
            electron_configuration: "[Rn]7s2 6d1".to_owned()
        });

        tmp.insert(90, Element { 
            number: 90, 
            name: "Thorium".to_owned(), 
            symbol: "Th".to_owned(),
            mass: 232.038_f32,
            electron_configuration: "[Rn]7s2 6d2".to_owned()
        });

        tmp.insert(91, Element { 
            number: 91, 
            name: "Protactinium".to_owned(), 
            symbol: "Pa".to_owned(),
            mass: 231.03588_f32,
            electron_configuration: "[Rn]7s2 5f2 6d1".to_owned()
        });

        tmp.insert(92, Element { 
            number: 92, 
            name: "Uranium".to_owned(), 
            symbol: "U".to_owned(),
            mass: 238.0289_f32,
            electron_configuration: "[Rn]7s2 5f3 6d1".to_owned()
        });

        tmp.insert(93, Element { 
            number: 93, 
            name: "Neptunium".to_owned(), 
            symbol: "Np".to_owned(),
            mass: 237.048172_f32,
            electron_configuration: "[Rn]7s2 5f4 6d1".to_owned()
        });

        tmp.insert(94, Element { 
            number: 94, 
            name: "Plutonium".to_owned(), 
            symbol: "Pu".to_owned(),
            mass: 244.06420_f32,
            electron_configuration: "[Rn]7s2 5f6".to_owned()
        });

        tmp.insert(95, Element { 
            number: 95, 
            name: "Americium".to_owned(), 
            symbol: "Am".to_owned(),
            mass: 243.061380_f32,
            electron_configuration: "[Rn]7s2 5f7".to_owned()
        });

        tmp.insert(96, Element { 
            number: 96, 
            name: "Curium".to_owned(), 
            symbol: "Cm".to_owned(),
            mass: 247.07035_f32,
            electron_configuration: "[Rn]7s2 5f7 6d1".to_owned()
        });

        tmp.insert(97, Element { 
            number: 97, 
            name: "Berkelium".to_owned(), 
            symbol: "Bk".to_owned(),
            mass: 247.07031_f32,
            electron_configuration: "[Rn]7s2 5f9".to_owned()
        });

        tmp.insert(98, Element { 
            number: 98, 
            name: "Californium".to_owned(), 
            symbol: "Cf".to_owned(),
            mass: 251.07959_f32,
            electron_configuration: "[Rn]7s2 5f10".to_owned()
        });

        tmp.insert(99, Element { 
            number: 99, 
            name: "Einsteinium".to_owned(), 
            symbol: "Es".to_owned(),
            mass: 252.0830_f32,
            electron_configuration: "[Rn]7s2 5f11".to_owned()
        });

        tmp.insert(100, Element { 
            number: 100, 
            name: "Fermium".to_owned(), 
            symbol: "Fm".to_owned(),
            mass: 257.09511_f32,
            electron_configuration: "[Rn] 5f12 7s2".to_owned()
        });

        tmp.insert(101, Element { 
            number: 101, 
            name: "Mendelevium".to_owned(), 
            symbol: "Md".to_owned(),
            mass: 258.09843_f32,
            electron_configuration: "[Rn]7s2 5f13".to_owned()
        });

        tmp.insert(102, Element { 
            number: 102, 
            name: "Nobelium".to_owned(), 
            symbol: "No".to_owned(),
            mass: 259.10100_f32,
            electron_configuration: "[Rn]7s2 5f14".to_owned()
        });

        tmp.insert(103, Element { 
            number: 103, 
            name: "Lawrencium".to_owned(), 
            symbol: "Lr".to_owned(),
            mass: 266.120_f32,
            electron_configuration: "[Rn]7s2 5f14 6d1".to_owned()
        });

        tmp.insert(104, Element { 
            number: 104, 
            name: "Rutherfordium".to_owned(), 
            symbol: "Rf".to_owned(),
            mass: 267.122_f32,
            electron_configuration: "[Rn]7s2 5f14 6d2".to_owned()
        });

        tmp.insert(105, Element { 
            number: 105, 
            name: "Dubnium".to_owned(), 
            symbol: "Db".to_owned(),
            mass: 268.126_f32,
            electron_configuration: "[Rn]7s2 5f14 6d3".to_owned()
        });

        tmp.insert(106, Element { 
            number: 106, 
            name: "Seaborgium".to_owned(), 
            symbol: "Sg".to_owned(),
            mass: 269.128_f32,
            electron_configuration: "[Rn]7s2 5f14 6d4".to_owned()
        });

        tmp.insert(107, Element { 
            number: 107, 
            name: "Bohrium".to_owned(), 
            symbol: "Bh".to_owned(),
            mass: 270.133_f32,
            electron_configuration: "[Rn]7s2 5f14 6d5".to_owned()
        });

        tmp.insert(108, Element { 
            number: 108, 
            name: "Hassium".to_owned(), 
            symbol: "Hs".to_owned(),
            mass: 269.1336_f32,
            electron_configuration: "[Rn]7s2 5f14 6d6".to_owned()
        });

        tmp.insert(109, Element { 
            number: 109, 
            name: "Meitnerium".to_owned(), 
            symbol: "Mt".to_owned(),
            mass: 277.154_f32,
            electron_configuration: "[Rn]7s2 5f14 6d7 (calculated)".to_owned()
        });

        tmp.insert(110, Element { 
            number: 110, 
            name: "Darmstadtium".to_owned(), 
            symbol: "Ds".to_owned(),
            mass: 282.166_f32,
            electron_configuration: "[Rn]7s2 5f14 6d8 (predicted)".to_owned()
        });

        tmp.insert(111, Element { 
            number: 111, 
            name: "Roentgenium".to_owned(), 
            symbol: "Rg".to_owned(),
            mass: 282.169_f32,
            electron_configuration: "[Rn]7s2 5f14 6d9 (predicted)".to_owned()
        });

        tmp.insert(112, Element { 
            number: 112, 
            name: "Copernicium".to_owned(), 
            symbol: "Cn".to_owned(),
            mass: 286.179_f32,
            electron_configuration: "[Rn]7s2 5f14 6d10 (predicted)".to_owned()
        });

        tmp.insert(113, Element { 
            number: 113, 
            name: "Nihonium".to_owned(), 
            symbol: "Nh".to_owned(),
            mass: 286.182_f32,
            electron_configuration: "[Rn]5f14 6d10 7s2 7p1 (predicted)".to_owned()
        });

        tmp.insert(114, Element { 
            number: 114, 
            name: "Flerovium".to_owned(), 
            symbol: "Fl".to_owned(),
            mass: 290.192_f32,
            electron_configuration: "[Rn]7s2 7p2 5f14 6d10 (predicted)".to_owned()
        });

        tmp.insert(115, Element { 
            number: 115, 
            name: "Moscovium".to_owned(), 
            symbol: "Mc".to_owned(),
            mass: 290.196_f32,
            electron_configuration: "[Rn]7s2 7p3 5f14 6d10 (predicted)".to_owned()
        });

        tmp.insert(116, Element { 
            number: 116, 
            name: "Livermorium".to_owned(), 
            symbol: "Lv".to_owned(),
            mass: 293.205_f32,
            electron_configuration: "[Rn]7s2 7p4 5f14 6d10 (predicted)".to_owned()
        });

        tmp.insert(117, Element { 
            number: 117, 
            name: "Tennessine".to_owned(), 
            symbol: "Ts".to_owned(),
            mass: 294.211_f32,
            electron_configuration: "[Rn]7s2 7p5 5f14 6d10 (predicted)".to_owned()
        });

        tmp.insert(118, Element { 
            number: 118, 
            name: "Oganesson".to_owned(), 
            symbol: "Og".to_owned(),
            mass: 295.216_f32,
            electron_configuration: "[Rn]7s2 7p6 5f14 6d10 (predicted)".to_owned()
        });
    tmp
    };
}

// INPUT TO ELEMENT - match Identifier to Element (proc macro this)
pub fn lookup(property: String) -> Result<&'static Element, &'static str> {
    match property.as_str() {
        "H" => Ok(&NUMBER_TO_ELEMENT[1]),
        "Hydrogen" => Ok(&NUMBER_TO_ELEMENT[1]),
        "1" => Ok(&NUMBER_TO_ELEMENT[1]),

        "He" => Ok(&NUMBER_TO_ELEMENT[2]),
        "Helium" => Ok(&NUMBER_TO_ELEMENT[2]),
        "2" => Ok(&NUMBER_TO_ELEMENT[2]),

        "Li" => Ok(&NUMBER_TO_ELEMENT[3]),
        "Lithium" => Ok(&NUMBER_TO_ELEMENT[3]),
        "3" => Ok(&NUMBER_TO_ELEMENT[3]),

        "Be" => Ok(&NUMBER_TO_ELEMENT[4]),
        "Beryllium" => Ok(&NUMBER_TO_ELEMENT[4]),
        "4" => Ok(&NUMBER_TO_ELEMENT[4]),

        "B" => Ok(&NUMBER_TO_ELEMENT[5]),
        "Boron" => Ok(&NUMBER_TO_ELEMENT[5]),
        "5" => Ok(&NUMBER_TO_ELEMENT[5]),

        "C" => Ok(&NUMBER_TO_ELEMENT[6]),
        "Carbon" => Ok(&NUMBER_TO_ELEMENT[6]),
        "6" => Ok(&NUMBER_TO_ELEMENT[6]),

        "N" => Ok(&NUMBER_TO_ELEMENT[7]),
        "Nitrogen" => Ok(&NUMBER_TO_ELEMENT[7]),
        "7" => Ok(&NUMBER_TO_ELEMENT[7]),

        "O" => Ok(&NUMBER_TO_ELEMENT[8]),
        "Oxygen" => Ok(&NUMBER_TO_ELEMENT[8]),
        "8" => Ok(&NUMBER_TO_ELEMENT[8]),

        "F" => Ok(&NUMBER_TO_ELEMENT[9]),
        "Fluorine" => Ok(&NUMBER_TO_ELEMENT[9]),
        "9" => Ok(&NUMBER_TO_ELEMENT[9]),

        "Ne" => Ok(&NUMBER_TO_ELEMENT[10]),
        "Neon" => Ok(&NUMBER_TO_ELEMENT[10]),
        "10" => Ok(&NUMBER_TO_ELEMENT[10]),

        "Na" => Ok(&NUMBER_TO_ELEMENT[11]),
        "Sodium" => Ok(&NUMBER_TO_ELEMENT[11]),
        "11" => Ok(&NUMBER_TO_ELEMENT[11]),

        "Mg" => Ok(&NUMBER_TO_ELEMENT[12]),
        "Magnesium" => Ok(&NUMBER_TO_ELEMENT[12]),
        "12" => Ok(&NUMBER_TO_ELEMENT[12]),

        "Al" => Ok(&NUMBER_TO_ELEMENT[13]),
        "Aluminum" => Ok(&NUMBER_TO_ELEMENT[13]),
        "13" => Ok(&NUMBER_TO_ELEMENT[13]),

        "Si" => Ok(&NUMBER_TO_ELEMENT[14]),
        "Silicon" => Ok(&NUMBER_TO_ELEMENT[14]),
        "14" => Ok(&NUMBER_TO_ELEMENT[14]),

        "P" => Ok(&NUMBER_TO_ELEMENT[15]),
        "Phosphorus" => Ok(&NUMBER_TO_ELEMENT[15]),
        "15" => Ok(&NUMBER_TO_ELEMENT[15]),

        "S" => Ok(&NUMBER_TO_ELEMENT[16]),
        "Sulfur" => Ok(&NUMBER_TO_ELEMENT[16]),
        "16" => Ok(&NUMBER_TO_ELEMENT[16]),

        "Cl" => Ok(&NUMBER_TO_ELEMENT[17]),
        "Chlorine" => Ok(&NUMBER_TO_ELEMENT[17]),
        "17" => Ok(&NUMBER_TO_ELEMENT[17]),

        "Ar" => Ok(&NUMBER_TO_ELEMENT[18]),
        "Argon" => Ok(&NUMBER_TO_ELEMENT[18]),
        "18" => Ok(&NUMBER_TO_ELEMENT[18]),

        "K" => Ok(&NUMBER_TO_ELEMENT[19]),
        "Potassium" => Ok(&NUMBER_TO_ELEMENT[19]),
        "19" => Ok(&NUMBER_TO_ELEMENT[19]),

        "Ca" => Ok(&NUMBER_TO_ELEMENT[20]),
        "Calcium" => Ok(&NUMBER_TO_ELEMENT[20]),
        "20" => Ok(&NUMBER_TO_ELEMENT[20]),

        "Sc" => Ok(&NUMBER_TO_ELEMENT[21]),
        "Scandium" => Ok(&NUMBER_TO_ELEMENT[21]),
        "21" => Ok(&NUMBER_TO_ELEMENT[21]),

        "Ti" => Ok(&NUMBER_TO_ELEMENT[22]),
        "Titanium" => Ok(&NUMBER_TO_ELEMENT[22]),
        "22" => Ok(&NUMBER_TO_ELEMENT[22]),

        "V" => Ok(&NUMBER_TO_ELEMENT[23]),
        "Vanadium" => Ok(&NUMBER_TO_ELEMENT[23]),
        "23" => Ok(&NUMBER_TO_ELEMENT[23]),

        "Cr" => Ok(&NUMBER_TO_ELEMENT[24]),
        "Chromium" => Ok(&NUMBER_TO_ELEMENT[24]),
        "24" => Ok(&NUMBER_TO_ELEMENT[24]),

        "Mn" => Ok(&NUMBER_TO_ELEMENT[25]),
        "Manganese" => Ok(&NUMBER_TO_ELEMENT[25]),
        "25" => Ok(&NUMBER_TO_ELEMENT[25]),

        "Fe" => Ok(&NUMBER_TO_ELEMENT[26]),
        "Iron" => Ok(&NUMBER_TO_ELEMENT[26]),
        "26" => Ok(&NUMBER_TO_ELEMENT[26]),

        "Co" => Ok(&NUMBER_TO_ELEMENT[27]),
        "Cobalt" => Ok(&NUMBER_TO_ELEMENT[27]),
        "27" => Ok(&NUMBER_TO_ELEMENT[27]),

        "Ni" => Ok(&NUMBER_TO_ELEMENT[28]),
        "Nickel" => Ok(&NUMBER_TO_ELEMENT[28]),
        "28" => Ok(&NUMBER_TO_ELEMENT[28]),

        "Cu" => Ok(&NUMBER_TO_ELEMENT[29]),
        "Copper" => Ok(&NUMBER_TO_ELEMENT[29]),
        "29" => Ok(&NUMBER_TO_ELEMENT[29]),

        "Zn" => Ok(&NUMBER_TO_ELEMENT[30]),
        "Zinc" => Ok(&NUMBER_TO_ELEMENT[30]),
        "30" => Ok(&NUMBER_TO_ELEMENT[30]),

        "Ga" => Ok(&NUMBER_TO_ELEMENT[31]),
        "Gallium" => Ok(&NUMBER_TO_ELEMENT[31]),
        "31" => Ok(&NUMBER_TO_ELEMENT[31]),

        "Ge" => Ok(&NUMBER_TO_ELEMENT[32]),
        "Germanium" => Ok(&NUMBER_TO_ELEMENT[32]),
        "32" => Ok(&NUMBER_TO_ELEMENT[32]),

        "As" => Ok(&NUMBER_TO_ELEMENT[33]),
        "Arsenic" => Ok(&NUMBER_TO_ELEMENT[33]),
        "33" => Ok(&NUMBER_TO_ELEMENT[33]),

        "Se" => Ok(&NUMBER_TO_ELEMENT[34]),
        "Selenium" => Ok(&NUMBER_TO_ELEMENT[34]),
        "34" => Ok(&NUMBER_TO_ELEMENT[34]),

        "Br" => Ok(&NUMBER_TO_ELEMENT[35]),
        "Bromine" => Ok(&NUMBER_TO_ELEMENT[35]),
        "35" => Ok(&NUMBER_TO_ELEMENT[35]),

        "Kr" => Ok(&NUMBER_TO_ELEMENT[36]),
        "Krypton" => Ok(&NUMBER_TO_ELEMENT[36]),
        "36" => Ok(&NUMBER_TO_ELEMENT[36]),

        "Rb" => Ok(&NUMBER_TO_ELEMENT[37]),
        "Rubidium" => Ok(&NUMBER_TO_ELEMENT[37]),
        "37" => Ok(&NUMBER_TO_ELEMENT[37]),

        "Sr" => Ok(&NUMBER_TO_ELEMENT[38]),
        "Strontium" => Ok(&NUMBER_TO_ELEMENT[38]),
        "38" => Ok(&NUMBER_TO_ELEMENT[38]),

        "Y" => Ok(&NUMBER_TO_ELEMENT[39]),
        "Yttrium" => Ok(&NUMBER_TO_ELEMENT[39]),
        "39" => Ok(&NUMBER_TO_ELEMENT[39]),

        "Zr" => Ok(&NUMBER_TO_ELEMENT[40]),
        "Zirconium" => Ok(&NUMBER_TO_ELEMENT[40]),
        "40" => Ok(&NUMBER_TO_ELEMENT[40]),

        "Nb" => Ok(&NUMBER_TO_ELEMENT[41]),
        "Niobium" => Ok(&NUMBER_TO_ELEMENT[41]),
        "41" => Ok(&NUMBER_TO_ELEMENT[41]),

        "Mo" => Ok(&NUMBER_TO_ELEMENT[42]),
        "Molybdenum" => Ok(&NUMBER_TO_ELEMENT[42]),
        "42" => Ok(&NUMBER_TO_ELEMENT[42]),

        "Tc" => Ok(&NUMBER_TO_ELEMENT[43]),
        "Technetium" => Ok(&NUMBER_TO_ELEMENT[43]),
        "43" => Ok(&NUMBER_TO_ELEMENT[43]),

        "Ru" => Ok(&NUMBER_TO_ELEMENT[44]),
        "Ruthenium" => Ok(&NUMBER_TO_ELEMENT[44]),
        "44" => Ok(&NUMBER_TO_ELEMENT[44]),

        "Rh" => Ok(&NUMBER_TO_ELEMENT[45]),
        "Rhodium" => Ok(&NUMBER_TO_ELEMENT[45]),
        "45" => Ok(&NUMBER_TO_ELEMENT[45]),

        "Pd" => Ok(&NUMBER_TO_ELEMENT[46]),
        "Palladium" => Ok(&NUMBER_TO_ELEMENT[46]),
        "46" => Ok(&NUMBER_TO_ELEMENT[46]),

        "Ag" => Ok(&NUMBER_TO_ELEMENT[47]),
        "Silver" => Ok(&NUMBER_TO_ELEMENT[47]),
        "47" => Ok(&NUMBER_TO_ELEMENT[47]),

        "Cd" => Ok(&NUMBER_TO_ELEMENT[48]),
        "Cadmium" => Ok(&NUMBER_TO_ELEMENT[48]),
        "48" => Ok(&NUMBER_TO_ELEMENT[48]),

        "In" => Ok(&NUMBER_TO_ELEMENT[49]),
        "Indium" => Ok(&NUMBER_TO_ELEMENT[49]),
        "49" => Ok(&NUMBER_TO_ELEMENT[49]),

        "Sn" => Ok(&NUMBER_TO_ELEMENT[50]),
        "Tin" => Ok(&NUMBER_TO_ELEMENT[50]),
        "50" => Ok(&NUMBER_TO_ELEMENT[50]),

        "Sb" => Ok(&NUMBER_TO_ELEMENT[51]),
        "Antimony" => Ok(&NUMBER_TO_ELEMENT[51]),
        "51" => Ok(&NUMBER_TO_ELEMENT[51]),

        "Te" => Ok(&NUMBER_TO_ELEMENT[52]),
        "Tellurium" => Ok(&NUMBER_TO_ELEMENT[52]),
        "52" => Ok(&NUMBER_TO_ELEMENT[52]),

        "I" => Ok(&NUMBER_TO_ELEMENT[53]),
        "Iodine" => Ok(&NUMBER_TO_ELEMENT[53]),
        "53" => Ok(&NUMBER_TO_ELEMENT[53]),

        "Xe" => Ok(&NUMBER_TO_ELEMENT[54]),
        "Xenon" => Ok(&NUMBER_TO_ELEMENT[54]),
        "54" => Ok(&NUMBER_TO_ELEMENT[54]),

        "Cs" => Ok(&NUMBER_TO_ELEMENT[55]),
        "Cesium" => Ok(&NUMBER_TO_ELEMENT[55]),
        "55" => Ok(&NUMBER_TO_ELEMENT[55]),

        "Ba" => Ok(&NUMBER_TO_ELEMENT[56]),
        "Barium" => Ok(&NUMBER_TO_ELEMENT[56]),
        "56" => Ok(&NUMBER_TO_ELEMENT[56]),

        "La" => Ok(&NUMBER_TO_ELEMENT[57]),
        "Lanthanum" => Ok(&NUMBER_TO_ELEMENT[57]),
        "57" => Ok(&NUMBER_TO_ELEMENT[57]),

        "Ce" => Ok(&NUMBER_TO_ELEMENT[58]),
        "Cerium" => Ok(&NUMBER_TO_ELEMENT[58]),
        "58" => Ok(&NUMBER_TO_ELEMENT[58]),

        "Pr" => Ok(&NUMBER_TO_ELEMENT[59]),
        "Praseodymium" => Ok(&NUMBER_TO_ELEMENT[59]),
        "59" => Ok(&NUMBER_TO_ELEMENT[59]),

        "Nd" => Ok(&NUMBER_TO_ELEMENT[60]),
        "Neodymium" => Ok(&NUMBER_TO_ELEMENT[60]),
        "60" => Ok(&NUMBER_TO_ELEMENT[60]),

        "Pm" => Ok(&NUMBER_TO_ELEMENT[61]),
        "Promethium" => Ok(&NUMBER_TO_ELEMENT[61]),
        "61" => Ok(&NUMBER_TO_ELEMENT[61]),

        "Sm" => Ok(&NUMBER_TO_ELEMENT[62]),
        "Samarium" => Ok(&NUMBER_TO_ELEMENT[62]),
        "62" => Ok(&NUMBER_TO_ELEMENT[62]),

        "Eu" => Ok(&NUMBER_TO_ELEMENT[63]),
        "Europium" => Ok(&NUMBER_TO_ELEMENT[63]),
        "63" => Ok(&NUMBER_TO_ELEMENT[63]),

        "Gd" => Ok(&NUMBER_TO_ELEMENT[64]),
        "Gadolinium" => Ok(&NUMBER_TO_ELEMENT[64]),
        "64" => Ok(&NUMBER_TO_ELEMENT[64]),

        "Tb" => Ok(&NUMBER_TO_ELEMENT[65]),
        "Terbium" => Ok(&NUMBER_TO_ELEMENT[65]),
        "65" => Ok(&NUMBER_TO_ELEMENT[65]),

        "Dy" => Ok(&NUMBER_TO_ELEMENT[66]),
        "Dysprosium" => Ok(&NUMBER_TO_ELEMENT[66]),
        "66" => Ok(&NUMBER_TO_ELEMENT[66]),

        "Ho" => Ok(&NUMBER_TO_ELEMENT[67]),
        "Holmium" => Ok(&NUMBER_TO_ELEMENT[67]),
        "67" => Ok(&NUMBER_TO_ELEMENT[67]),

        "Er" => Ok(&NUMBER_TO_ELEMENT[68]),
        "Erbium" => Ok(&NUMBER_TO_ELEMENT[68]),
        "68" => Ok(&NUMBER_TO_ELEMENT[68]),

        "Tm" => Ok(&NUMBER_TO_ELEMENT[69]),
        "Thulium" => Ok(&NUMBER_TO_ELEMENT[69]),
        "69" => Ok(&NUMBER_TO_ELEMENT[69]),

        "Yb" => Ok(&NUMBER_TO_ELEMENT[70]),
        "Ytterbium" => Ok(&NUMBER_TO_ELEMENT[70]),
        "70" => Ok(&NUMBER_TO_ELEMENT[70]),

        "Lu" => Ok(&NUMBER_TO_ELEMENT[71]),
        "Lutetium" => Ok(&NUMBER_TO_ELEMENT[71]),
        "71" => Ok(&NUMBER_TO_ELEMENT[71]),

        "Hf" => Ok(&NUMBER_TO_ELEMENT[72]),
        "Hafnium" => Ok(&NUMBER_TO_ELEMENT[72]),
        "72" => Ok(&NUMBER_TO_ELEMENT[72]),

        "Ta" => Ok(&NUMBER_TO_ELEMENT[73]),
        "Tantalum" => Ok(&NUMBER_TO_ELEMENT[73]),
        "73" => Ok(&NUMBER_TO_ELEMENT[73]),

        "W" => Ok(&NUMBER_TO_ELEMENT[74]),
        "Tungsten" => Ok(&NUMBER_TO_ELEMENT[74]),
        "74" => Ok(&NUMBER_TO_ELEMENT[74]),

        "Re" => Ok(&NUMBER_TO_ELEMENT[75]),
        "Rhenium" => Ok(&NUMBER_TO_ELEMENT[75]),
        "75" => Ok(&NUMBER_TO_ELEMENT[75]),

        "Os" => Ok(&NUMBER_TO_ELEMENT[76]),
        "Osmium" => Ok(&NUMBER_TO_ELEMENT[76]),
        "76" => Ok(&NUMBER_TO_ELEMENT[76]),

        "Ir" => Ok(&NUMBER_TO_ELEMENT[77]),
        "Iridium" => Ok(&NUMBER_TO_ELEMENT[77]),
        "77" => Ok(&NUMBER_TO_ELEMENT[77]),

        "Pt" => Ok(&NUMBER_TO_ELEMENT[78]),
        "Platinum" => Ok(&NUMBER_TO_ELEMENT[78]),
        "78" => Ok(&NUMBER_TO_ELEMENT[78]),

        "Au" => Ok(&NUMBER_TO_ELEMENT[79]),
        "Gold" => Ok(&NUMBER_TO_ELEMENT[79]),
        "79" => Ok(&NUMBER_TO_ELEMENT[79]),

        "Hg" => Ok(&NUMBER_TO_ELEMENT[80]),
        "Mercury" => Ok(&NUMBER_TO_ELEMENT[80]),
        "80" => Ok(&NUMBER_TO_ELEMENT[80]),

        "Tl" => Ok(&NUMBER_TO_ELEMENT[81]),
        "Thallium" => Ok(&NUMBER_TO_ELEMENT[81]),
        "81" => Ok(&NUMBER_TO_ELEMENT[81]),

        "Pb" => Ok(&NUMBER_TO_ELEMENT[82]),
        "Lead" => Ok(&NUMBER_TO_ELEMENT[82]),
        "82" => Ok(&NUMBER_TO_ELEMENT[82]),

        "Bi" => Ok(&NUMBER_TO_ELEMENT[83]),
        "Bismuth" => Ok(&NUMBER_TO_ELEMENT[83]),
        "83" => Ok(&NUMBER_TO_ELEMENT[83]),

        "Po" => Ok(&NUMBER_TO_ELEMENT[84]),
        "Polonium" => Ok(&NUMBER_TO_ELEMENT[84]),
        "84" => Ok(&NUMBER_TO_ELEMENT[84]),

        "At" => Ok(&NUMBER_TO_ELEMENT[85]),
        "Astatine" => Ok(&NUMBER_TO_ELEMENT[85]),
        "85" => Ok(&NUMBER_TO_ELEMENT[85]),

        "Rn" => Ok(&NUMBER_TO_ELEMENT[86]),
        "Radon" => Ok(&NUMBER_TO_ELEMENT[86]),
        "86" => Ok(&NUMBER_TO_ELEMENT[86]),

        "Fr" => Ok(&NUMBER_TO_ELEMENT[87]),
        "Francium" => Ok(&NUMBER_TO_ELEMENT[87]),
        "87" => Ok(&NUMBER_TO_ELEMENT[87]),

        "Ra" => Ok(&NUMBER_TO_ELEMENT[88]),
        "Radium" => Ok(&NUMBER_TO_ELEMENT[88]),
        "88" => Ok(&NUMBER_TO_ELEMENT[88]),

        "Ac" => Ok(&NUMBER_TO_ELEMENT[89]),
        "Actinium" => Ok(&NUMBER_TO_ELEMENT[89]),
        "89" => Ok(&NUMBER_TO_ELEMENT[89]),

        "Th" => Ok(&NUMBER_TO_ELEMENT[90]),
        "Thorium" => Ok(&NUMBER_TO_ELEMENT[90]),
        "90" => Ok(&NUMBER_TO_ELEMENT[90]),

        "Pa" => Ok(&NUMBER_TO_ELEMENT[91]),
        "Protactinium" => Ok(&NUMBER_TO_ELEMENT[91]),
        "91" => Ok(&NUMBER_TO_ELEMENT[91]),

        "U" => Ok(&NUMBER_TO_ELEMENT[92]),
        "Uranium" => Ok(&NUMBER_TO_ELEMENT[92]),
        "92" => Ok(&NUMBER_TO_ELEMENT[92]),

        "Np" => Ok(&NUMBER_TO_ELEMENT[93]),
        "Neptunium" => Ok(&NUMBER_TO_ELEMENT[93]),
        "93" => Ok(&NUMBER_TO_ELEMENT[93]),

        "Pu" => Ok(&NUMBER_TO_ELEMENT[94]),
        "Plutonium" => Ok(&NUMBER_TO_ELEMENT[94]),
        "94" => Ok(&NUMBER_TO_ELEMENT[94]),

        "Am" => Ok(&NUMBER_TO_ELEMENT[95]),
        "Americium" => Ok(&NUMBER_TO_ELEMENT[95]),
        "95" => Ok(&NUMBER_TO_ELEMENT[95]),

        "Cm" => Ok(&NUMBER_TO_ELEMENT[96]),
        "Curium" => Ok(&NUMBER_TO_ELEMENT[96]),
        "96" => Ok(&NUMBER_TO_ELEMENT[96]),

        "Bk" => Ok(&NUMBER_TO_ELEMENT[97]),
        "Berkelium" => Ok(&NUMBER_TO_ELEMENT[97]),
        "97" => Ok(&NUMBER_TO_ELEMENT[97]),

        "Cf" => Ok(&NUMBER_TO_ELEMENT[98]),
        "Californium" => Ok(&NUMBER_TO_ELEMENT[98]),
        "98" => Ok(&NUMBER_TO_ELEMENT[98]),

        "Es" => Ok(&NUMBER_TO_ELEMENT[99]),
        "Einsteinium" => Ok(&NUMBER_TO_ELEMENT[99]),
        "99" => Ok(&NUMBER_TO_ELEMENT[99]),

        "Fm" => Ok(&NUMBER_TO_ELEMENT[100]),
        "Fermium" => Ok(&NUMBER_TO_ELEMENT[100]),
        "100" => Ok(&NUMBER_TO_ELEMENT[100]),

        "Md" => Ok(&NUMBER_TO_ELEMENT[101]),
        "Mendelevium" => Ok(&NUMBER_TO_ELEMENT[101]),
        "101" => Ok(&NUMBER_TO_ELEMENT[101]),

        "No" => Ok(&NUMBER_TO_ELEMENT[102]),
        "Nobelium" => Ok(&NUMBER_TO_ELEMENT[102]),
        "102" => Ok(&NUMBER_TO_ELEMENT[102]),

        "Lr" => Ok(&NUMBER_TO_ELEMENT[103]),
        "Lawrencium" => Ok(&NUMBER_TO_ELEMENT[103]),
        "103" => Ok(&NUMBER_TO_ELEMENT[103]),

        "Rf" => Ok(&NUMBER_TO_ELEMENT[104]),
        "Rutherfordium" => Ok(&NUMBER_TO_ELEMENT[104]),
        "104" => Ok(&NUMBER_TO_ELEMENT[104]),

        "Db" => Ok(&NUMBER_TO_ELEMENT[105]),
        "Dubnium" => Ok(&NUMBER_TO_ELEMENT[105]),
        "105" => Ok(&NUMBER_TO_ELEMENT[105]),

        "Sg" => Ok(&NUMBER_TO_ELEMENT[106]),
        "Seaborgium" => Ok(&NUMBER_TO_ELEMENT[106]),
        "106" => Ok(&NUMBER_TO_ELEMENT[106]),

        "Bh" => Ok(&NUMBER_TO_ELEMENT[107]),
        "Bohrium" => Ok(&NUMBER_TO_ELEMENT[107]),
        "107" => Ok(&NUMBER_TO_ELEMENT[107]),

        "Hs" => Ok(&NUMBER_TO_ELEMENT[108]),
        "Hassium" => Ok(&NUMBER_TO_ELEMENT[108]),
        "108" => Ok(&NUMBER_TO_ELEMENT[108]),

        "Mt" => Ok(&NUMBER_TO_ELEMENT[109]),
        "Meitnerium" => Ok(&NUMBER_TO_ELEMENT[109]),
        "109" => Ok(&NUMBER_TO_ELEMENT[109]),

        "Ds" => Ok(&NUMBER_TO_ELEMENT[110]),
        "Darmstadtium" => Ok(&NUMBER_TO_ELEMENT[110]),
        "110" => Ok(&NUMBER_TO_ELEMENT[110]),

        "Rg" => Ok(&NUMBER_TO_ELEMENT[111]),
        "Roentgenium" => Ok(&NUMBER_TO_ELEMENT[111]),
        "111" => Ok(&NUMBER_TO_ELEMENT[111]),

        "Cn" => Ok(&NUMBER_TO_ELEMENT[112]),
        "Copernicium" => Ok(&NUMBER_TO_ELEMENT[112]),
        "112" => Ok(&NUMBER_TO_ELEMENT[112]),

        "Nh" => Ok(&NUMBER_TO_ELEMENT[113]),
        "Nihonium" => Ok(&NUMBER_TO_ELEMENT[113]),
        "113" => Ok(&NUMBER_TO_ELEMENT[113]),

        "Fl" => Ok(&NUMBER_TO_ELEMENT[114]),
        "Flerovium" => Ok(&NUMBER_TO_ELEMENT[114]),
        "114" => Ok(&NUMBER_TO_ELEMENT[114]),

        "Mc" => Ok(&NUMBER_TO_ELEMENT[115]),
        "Moscovium" => Ok(&NUMBER_TO_ELEMENT[115]),
        "115" => Ok(&NUMBER_TO_ELEMENT[115]),

        "Lv" => Ok(&NUMBER_TO_ELEMENT[116]),
        "Livermorium" => Ok(&NUMBER_TO_ELEMENT[116]),
        "116" => Ok(&NUMBER_TO_ELEMENT[116]),

        "Ts" => Ok(&NUMBER_TO_ELEMENT[117]),
        "Tennessine" => Ok(&NUMBER_TO_ELEMENT[117]),
        "117" => Ok(&NUMBER_TO_ELEMENT[117]),

        "Og" => Ok(&NUMBER_TO_ELEMENT[118]),
        "Oganesson" => Ok(&NUMBER_TO_ELEMENT[118]),
        "118" => Ok(&NUMBER_TO_ELEMENT[118]),

        _ => Err("Invalid element identifier."),
    }
}