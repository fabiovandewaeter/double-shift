use crate::metier::personnage::Stats;

pub struct Item {
    pub nom: String,
    pub stats: Stats,
}

impl Item {
    pub fn new(nom: String, stats: Stats) -> Self {
        Self { nom, stats }
    }
}
