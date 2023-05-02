use crate::elements::Element;
use colored::Colorize;

const PERIODIC_TABLE:&str = " __________________________________________________________________________ 
|   1   2   3   4   5   6   7   8   9   10  11  12  13  14  15  16  17  18 |
|1  H                                                                   He |
|2  Li  Be                                          B   C   N   O   F   Ne |
|3  Na  Mg                                          Al  Si  P   S   Cl  Ar |
|4  K   Ca  Sc  Ti  V   Cr  Mn  Fe  Co  Ni  Cu  Zn  Ga  Ge  As  Se  Br  Kr |
|5  Rb  Sr  Y   Zr  Nb  Mo  Tc  Ru  Rh  Pd  Ag  Cd  In  Sn  Sb  Te  I   Xe |
|6  Cs  Ba  *   Hf  Ta  W   Re  Os  Ir  Pt  Au  Hg  Tl  Pb  Bi  Po  At  Rn |
|7  Fr  Ra  **  Rf  Db  Sg  Bh  Hs  Mt  Ds  Rg  Cn  Nh  Fl  Mc  Lv  Ts  Og |
|                                                                          |
| Lanthanide*   La  Ce  Pr  Nd  Pm  Sm  Eu  Gd  Tb  Dy  Ho  Er  Tm  Yb  Lu |
|   Actinide**  Ac  Th  Pa  U   Np  Pu  Am  Cm  Bk  Cf  Es  Fm  Md  No  Lr |
|__________________________________________________________________________|";

const MOL:f32 = 6.02214076e23;
const LITRE:f32 = 22.4;

pub fn molar_mass(elements: &Vec<(&Element, u32)>) -> f32 {
    let mut mass: f32 = 0.0;
    for (element, quantity) in elements {
        mass += element.mass * quantity.clone() as f32;
    }
    mass
}

pub fn mol_to_litre(amount: f32) -> f32 {
    amount * LITRE
}

pub fn mol_to_gram(elements: &Vec<(Element, u32)>) -> f32 {
    let mut grams:f32 = 0.0;
    for (element, quantity) in elements {
        grams += quantity.clone() as f32 * MOL / element.mass as f32;
    }
    grams
}

pub fn mol_to_part(amount: u32) -> f32 {
    amount as f32 * MOL
}

pub fn litre_to_mol(amount: f32) -> f32 {
    amount / LITRE
}

pub fn gram_to_mol(elements: &Vec<(Element, u32)>) -> f32 {
    let mut moles:f32 = 0.0;
    for (element, quantity) in elements {
        moles += quantity.clone() as f32 * element.mass as f32 / MOL;
    }
    moles
}

pub fn part_to_mol(amount: f32) -> f32 {
    amount / MOL
}

pub fn build_compounds(elements: &Vec<(&Element, u32)>) -> String {
    elements.iter().map(|(element, quantity)| format!("{}{}", element.symbol, quantity)).collect::<Vec<String>>().join("")
}

pub fn print_molar_mass(elements: &Vec<(&Element, u32)>) {
    println!("Molar mass: {} g{}/mol", molar_mass(&elements), build_compounds(&elements));
}

pub fn print_periodic_table() {
    println!("{}", PERIODIC_TABLE.blue());
}
