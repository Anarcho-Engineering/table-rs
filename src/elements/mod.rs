use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Element {
    pub number: i32,
    pub name: &'static str,
    pub symbol: &'static str,
    pub mass: f32,
    pub electron_configuration: &'static str,
}

macro_rules! build_elements {
    ( $len:literal $i:ident ; $( $sym:ident $num:literal : $name:ident $mass:literal $ec:literal ),* ) => {
        pub static $i: [Element; $len] = [$( Element{ number: $num, name: stringify!($name), symbol: stringify!($sym), mass: $mass, electron_configuration: $ec } ),*];

        pub fn lookup(property: String) -> Result<&'static Element, &'static str> {
            match property.as_str() {
                $(
                    stringify!($sym) | stringify!($name) | stringify!($num) => Ok(&$i[$num - 1]),
                )*
                _ => Err("Invalid element identifier.")
            }
        }
    };
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

build_elements! {118 ELEMENTS;
    H  1:   Hydrogen        1.00800000 "1s1",
    He 2:   Helium          4.00260000 "1s2",
    Li 3:   Lithium         7.00000000 "[He]2s1",
    Be 4:   Beryllium       9.01218300 "[He]2s2",
    B  5:   Boron          10.81000000 "[He]2s2 2p1",
    C  6:   Carbon         12.01100000 "[He]2s2 2p2",
    N  7:   Nitrogen       14.00700000 "[He] 2s2 2p3",
    O  8:   Oxygen         15.99900000 "[He]2s2 2p4",
    F  9:   Fluorine       18.99840316 "[He]2s2 2p5",
    Ne 10:  Neon           20.18000000 "[He]2s2 2p6",
    Na 11:  Sodium         22.98976930 "[Ne]3s1",
    Mg 12:  Magnesium      24.30500000 "[Ne]3s2",
    Al 13:  Aluminum       26.98153800 "[Ne]3s2 3p1",
    Si 14:  Silicon        28.08500000 "[Ne]3s2 3p2",
    P  15:  Phosphorus     30.97376200 "[Ne]3s2 3p3",
    S  16:  Sulfur         32.07000000 "[Ne]3s2 3p4",
    Cl 17:  Chlorine       35.45000000 "[Ne]3s2 3p5",
    Ar 18:  Argon          39.90000000 "[Ne]3s2 3p6",
    K  19:  Potassium      39.09830000 "[Ar]4s1",
    Ca 20:  Calcium        40.08000000 "[Ar]4s2",
    Sc 21:  Scandium       44.95591000 "[Ar]4s2 3d1",
    Ti 22:  Titanium       47.86700000 "[Ar]4s2 3d2",
    V  23:  Vanadium       50.94150000 "[Ar]4s2 3d3",
    Cr 24:  Chromium       51.99600000 "[Ar]3d5 4s1",
    Mn 25:  Manganese      54.93804000 "[Ar]4s2 3d5",
    Fe 26:  Iron           55.84000000 "[Ar]4s2 3d6",
    Co 27:  Cobalt         58.93319000 "[Ar]4s2 3d7",
    Ni 28:  Nickel         58.69300000 "[Ar]4s2 3d8",
    Cu 29:  Copper         63.55000000 "[Ar]4s1 3d10",
    Zn 30:  Zinc           65.40000000 "[Ar]4s2 3d10",
    Ga 31:  Gallium        69.72300000 "[Ar]4s2 3d10 4p1",
    Ge 32:  Germanium      72.63000000 "[Ar]4s2 3d10 4p2",
    As 33:  Arsenic        74.92159000 "[Ar]4s2 3d10 4p3",
    Se 34:  Selenium       78.97000000 "[Ar]4s2 3d10 4p4",
    Br 35:  Bromine        79.90000000 "[Ar]4s2 3d10 4p5",
    Kr 36:  Krypton        83.80000000 "[Ar]4s2 3d10 4p6",
    Rb 37:  Rubidium       85.46800000 "[Kr]5s1",
    Sr 38:  Strontium      87.62000000 "[Kr]5s2",
    Y  39:  Yttrium        88.90584000 "[Kr]5s2 4d1",
    Zr 40:  Zirconium      91.22000000 "[Kr]5s2 4d2",
    Nb 41:  Niobium        92.90637000 "[Kr]5s1 4d4",
    Mo 42:  Molybdenum     95.95000000 "[Kr]5s1 4d5",
    Tc 43:  Technetium     96.90636000 "[Kr]5s2 4d5",
    Ru 44:  Ruthenium     101.10000000 "[Kr]5s1 4d7",
    Rh 45:  Rhodium       102.90550000 "[Kr]5s1 4d8",
    Pd 46:  Palladium     106.42000000 "[Kr]4d10",
    Ag 47:  Silver        107.86800000 "[Kr]5s1 4d10",
    Cd 48:  Cadmium       112.41000000 "[Kr]5s2 4d10",
    In 49:  Indium        114.81800000 "[Kr]5s2 4d10 5p1",
    Sn 50:  Tin           118.71000000 "[Kr]5s2 4d10 5p2",
    Sb 51:  Antimony      121.76000000 "[Kr]5s2 4d10 5p3",
    Te 52:  Tellurium     127.60000000 "[Kr]5s2 4d10 5p4",
    I  53:  Iodine        126.90450000 "[Kr]5s2 4d10 5p5",
    Xe 54:  Xenon         131.29000000 "[Kr]5s2 4d10 5p6",
    Cs 55:  Cesium        132.90545200 "[Xe]6s1",
    Ba 56:  Barium        137.33000000 "[Xe]6s2",
    La 57:  Lanthanum     138.90550000 "[Xe]6s2 5d1",
    Ce 58:  Cerium        140.11600000 "[Xe]6s2 4f1 5d1",
    Pr 59:  Praseodymium  140.90766000 "[Xe]6s2 4f3",
    Nd 60:  Neodymium     144.24000000 "[Xe]6s2 4f4",
    Pm 61:  Promethium    144.91276000 "[Xe]6s2 4f5",
    Sm 62:  Samarium      150.40000000 "[Xe]6s2 4f6",
    Eu 63:  Europium      151.96400000 "[Xe]6s2 4f7",
    Gd 64:  Gadolinium    157.20000000 "[Xe]6s2 4f7 5d1",
    Tb 65:  Terbium       158.92535000 "[Xe]6s2 4f9",
    Dy 66:  Dysprosium    162.50000000 "[Xe]6s2 4f10",
    Ho 67:  Holmium       164.93033000 "[Xe]6s2 4f11",
    Er 68:  Erbium        167.26000000 "[Xe]6s2 4f12",
    Tm 69:  Thulium       168.93422000 "[Xe]6s2 4f13",
    Yb 70:  Ytterbium     173.05000000 "[Xe]6s2 4f14",
    Lu 71:  Lutetium      174.96680000 "[Xe]6s2 4f14 5d1",
    Hf 72:  Hafnium       178.49000000 "[Xe]6s2 4f14 5d2",
    Ta 73:  Tantalum      180.94790000 "[Xe]6s2 4f14 5d3",
    W  74:  Tungsten      183.84000000 "[Xe]6s2 4f14 5d4",
    Re 75:  Rhenium       186.20700000 "[Xe]6s2 4f14 5d5",
    Os 76:  Osmium        190.20000000 "[Xe]6s2 4f14 5d6",
    Ir 77:  Iridium       192.22000000 "[Xe]6s2 4f14 5d7",
    Pt 78:  Platinum      195.08000000 "[Xe]6s1 4f14 5d9",
    Au 79:  Gold          196.96657000 "[Xe]6s1 4f14 5d10",
    Hg 80:  Mercury       200.59000000 "[Xe]6s2 4f14 5d10",
    Tl 81:  Thallium      204.38300000 "[Xe]6s2 4f14 5d10 6p1",
    Pb 82:  Lead          207.00000000 "[Xe]6s2 4f14 5d10 6p2",
    Bi 83:  Bismuth       208.98040000 "[Xe]6s2 4f14 5d10 6p3",
    Po 84:  Polonium      208.98243000 "[Xe]6s2 4f14 5d10 6p4",
    At 85:  Astatine      209.98715000 "[Xe]6s2 4f14 5d10 6p5",
    Rn 86:  Radon         222.01758000 "[Xe]6s2 4f14 5d10 6p6",
    Fr 87:  Francium      223.01973000 "[Rn]7s1",
    Ra 88:  Radium        226.02541000 "[Rn]7s2",
    Ac 89:  Actinium      227.02775000 "[Rn]7s2 6d1",
    Th 90:  Thorium       232.03800000 "[Rn]7s2 6d2",
    Pa 91:  Protactinium  231.03588000 "[Rn]7s2 5f2 6d1",
    U  92:  Uranium       238.02890000 "[Rn]7s2 5f3 6d1",
    Np 93:  Neptunium     237.04817200 "[Rn]7s2 5f4 6d1",
    Pu 94:  Plutonium     244.06420000 "[Rn]7s2 5f6",
    Am 95:  Americium     243.06138000 "[Rn]7s2 5f7",
    Cm 96:  Curium        247.07035000 "[Rn]7s2 5f7 6d1",
    Bk 97:  Berkelium     247.07031000 "[Rn]7s2 5f9",
    Cf 98:  Californium   251.07959000 "[Rn]7s2 5f10",
    Es 99:  Einsteinium   252.08300000 "[Rn]7s2 5f11",
    Fm 100: Fermium       257.09511000 "[Rn] 5f12 7s2",
    Md 101: Mendelevium   258.09843000 "[Rn]7s2 5f13",
    No 102: Nobelium      259.10100000 "[Rn]7s2 5f14",
    Lr 103: Lawrencium    266.12000000 "[Rn]7s2 5f14 6d1",
    Rf 104: Rutherfordium 267.12200000 "[Rn]7s2 5f14 6d2",
    Db 105: Dubnium       268.12600000 "[Rn]7s2 5f14 6d3",
    Sg 106: Seaborgium    269.12800000 "[Rn]7s2 5f14 6d4",
    Bh 107: Bohrium       270.13300000 "[Rn]7s2 5f14 6d5",
    Hs 108: Hassium       269.13360000 "[Rn]7s2 5f14 6d6",
    Mt 109: Meitnerium    277.15400000 "[Rn]7s2 5f14 6d7",
    Ds 110: Darmstadtium  282.16600000 "[Rn]7s2 5f14 6d8",
    Rg 111: Roentgenium   282.16900000 "[Rn]7s2 5f14 6d9",
    Cn 112: Copernicium   286.17900000 "[Rn]7s2 5f14 6d10",
    Nh 113: Nihonium      286.18200000 "[Rn]5f14 6d10 7s2 7p1",
    Fl 114: Flerovium     290.19200000 "[Rn]7s2 7p2 5f14 6d10",
    Mc 115: Moscovium     290.19600000 "[Rn]7s2 7p3 5f14 6d10",
    Lv 116: Livermorium   293.20500000 "[Rn]7s2 7p4 5f14 6d10",
    Ts 117: Tennessine    294.21100000 "[Rn]7s2 7p5 5f14 6d10",
    Og 118: Oganesson     295.21600000 "[Rn]7s2 7p6 5f14 6d10"
}
