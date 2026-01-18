// #[derive(Clone, Debug, PartialEq)]
// pub enum Etat {
//     Vivant,
//     Mort,
//     //Endormi
// }

pub struct StatsPersonnage {
    pub pv_max: i32,
    pub attaque: u32,
}

#[derive(Clone, Debug)]
pub struct Personnage {
    pub nom: String,
    pub pv: i32,
    pub pv_max: i32,
    pub attaque: u32,
}

impl Personnage {
    pub fn new(nom: String, stats: StatsPersonnage) -> Self {
        Self {
            nom,
            pv: stats.pv_max,
            pv_max: stats.pv_max,
            attaque: stats.attaque,
        }
    }
}
