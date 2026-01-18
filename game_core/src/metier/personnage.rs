// #[derive(Clone, Debug, PartialEq)]
// pub enum Etat {
//     Vivant,
//     Mort,
//     //Endormi
// }

#[derive(Clone, Debug)]
pub struct Personnage {
    pub nom: String,
    pub pv: i32,
    pub pv_max: i32,
}

impl Personnage {
    pub fn new(nom: String, pv_max: i32) -> Self {
        Self {
            nom,
            pv: pv_max,
            pv_max,
        }
    }
}
