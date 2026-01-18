// #[derive(Clone, Debug, PartialEq)]
// pub enum Etat {
//     Vivant,
//     Mort,
//     //Endormi
// }

#[derive(Clone)]
pub struct Stats {
    pub pv: i32,
    pub pv_max: i32,
    pub attaque: u32,
}

impl Stats {
    pub fn new(pv_max: i32, attaque: u32) -> Self {
        Self {
            pv: pv_max,
            pv_max,
            attaque,
        }
    }
}

#[derive(Clone)]
pub struct Personnage {
    pub nom: String,
    pub stats: Stats,
}

impl Personnage {
    pub fn new(nom: String, stats: Stats) -> Self {
        Self { nom, stats }
    }
}
